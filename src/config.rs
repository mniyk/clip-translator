// src/config.rs

use clap::Parser;

/// クリップボード監視で英⇄日翻訳を行う常駐ツール。
#[derive(Debug, Clone, Parser)]
#[command(name = "clip-translator", version, about)]
pub struct CliArgs {
    /// Ollama サーバーのエンドポイント (例: http://192.168.0.50:11434)
    #[arg(long)]
    pub endpoint: String,

    /// 使用する Ollama モデル名 (例: llama3.1, gpt-oss:20b)
    #[arg(long)]
    pub model: String,
}

/// Ollama リクエストのタイムアウト (秒)。
pub const OLLAMA_TIMEOUT_SEC: u64 = 30;

/// クリップボード監視のポーリング間隔 (ミリ秒)。
pub const POLL_INTERVAL_MS: u64 = 500;

/// 翻訳対象の最小文字数。
pub const MIN_CHARS: usize = 3;

/// 翻訳対象の最大文字数。
pub const MAX_CHARS: usize = 2000;

/// 通知タイトル。
pub const NOTIFICATION_TITLE: &str = "Clip Translator";

/// 通知の表示時間 (ミリ秒、OS依存)。
pub const NOTIFICATION_TIMEOUT_MS: u32 = 5000;

/// Pause/Resume をトグルするホットキー。
pub const TOGGLE_PAUSE_HOTKEY: &str = "Ctrl+Alt+P";
