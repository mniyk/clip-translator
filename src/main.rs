// src/main.rs

mod clipboard;
mod config;
mod hotkey;
mod notifier;
mod state;
mod tray;
mod translator;

use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use clap::Parser;
use tao::event::Event;
use tao::event_loop::{ControlFlow, EventLoop};
use tracing::{error, info, warn};

use crate::clipboard::ClipboardWatcher;
use crate::config::{CliArgs, OLLAMA_TIMEOUT_SEC, POLL_INTERVAL_MS, TOGGLE_PAUSE_HOTKEY};
use crate::state::AppState;
use crate::translator::Translator;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // CLI 引数パース。--endpoint と --model 必須。
    let args = CliArgs::parse();

    info!("starting clip-translator");
    info!(
        endpoint = %args.endpoint,
        model = %args.model,
        "ollama settings"
    );
    info!(toggle_pause = TOGGLE_PAUSE_HOTKEY, "hotkey settings");

    let state = AppState::new();

    // バックグラウンドスレッドでクリップボード監視。
    let bg_state = state.clone();
    let bg_args = args.clone();
    std::thread::Builder::new()
        .name("clipboard-loop".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build tokio runtime");

            rt.block_on(async move {
                if let Err(e) = run_clipboard_loop(bg_args, bg_state).await {
                    error!(error = %e, "clipboard loop terminated with error");
                }
            });
        })
        .context("failed to spawn clipboard-loop thread")?;

    // メインスレッドで tao の EventLoop を起動。
    let event_loop = EventLoop::new();

    // トレイアイコン構築。
    let tray = tray::Tray::new(&state).context("failed to init tray")?;

    // グローバルホットキー登録。
    let hotkey = hotkey::Hotkey::new().context("failed to init hotkey")?;

    let menu_channel = tray_icon::menu::MenuEvent::receiver();
    let hotkey_channel = global_hotkey::GlobalHotKeyEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(100));

        if let Ok(menu_event) = menu_channel.try_recv() {
            match tray.classify(&menu_event) {
                tray::TrayAction::TogglePause => {
                    let now_paused = state.toggle();
                    tray.update_pause_label(now_paused);
                    info!(paused = now_paused, "toggled pause state via tray");
                }
                tray::TrayAction::Quit => {
                    info!("quit requested from tray");
                    *control_flow = ControlFlow::Exit;
                }
                tray::TrayAction::Other => {}
            }
        }

        if let Ok(hotkey_event) = hotkey_channel.try_recv() {
            if hotkey_event.state == global_hotkey::HotKeyState::Pressed {
                match hotkey.classify(&hotkey_event) {
                    hotkey::HotkeyAction::TogglePause => {
                        let now_paused = state.toggle();
                        tray.update_pause_label(now_paused);
                        info!(paused = now_paused, "toggled pause state via hotkey");
                    }
                    hotkey::HotkeyAction::Other => {}
                }
            }
        }

        if let Event::LoopDestroyed = event {
            info!("event loop destroyed, exiting");
        }
    });
}

async fn run_clipboard_loop(args: CliArgs, state: AppState) -> Result<()> {
    let translator = Translator::new(&args).context("failed to init translator")?;
    let mut watcher = ClipboardWatcher::new().context("failed to init clipboard watcher")?;

    let poll_interval = Duration::from_millis(POLL_INTERVAL_MS);
    let llm_timeout = Duration::from_secs(OLLAMA_TIMEOUT_SEC);

    info!("entering main loop");

    loop {
        tokio::time::sleep(poll_interval).await;

        if state.is_paused() {
            continue;
        }

        let text = match watcher.poll() {
            Some(t) => t,
            None => continue,
        };

        info!(len = text.chars().count(), "translating clipboard text");

        let result = tokio::time::timeout(llm_timeout, translator.translate(&text)).await;

        let translated = match result {
            Ok(Ok(Some(t))) => t,
            Ok(Ok(None)) => {
                info!("LLM returned SKIP, ignoring");
                continue;
            }
            Ok(Err(e)) => {
                warn!(error = %e, "translation failed");
                continue;
            }
            Err(_) => {
                warn!(timeout_sec = OLLAMA_TIMEOUT_SEC, "translation timed out");
                continue;
            }
        };

        let body = format!("{}\n{}", truncate(&text, 60), translated);

        if let Err(e) = notifier::notify(&body) {
            error!(error = %e, "failed to show notification");
        }
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let mut t: String = chars.into_iter().take(max_chars).collect();
        t.push('…');
        t
    }
}
