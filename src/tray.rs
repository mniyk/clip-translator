// src/tray.rs

use anyhow::{Context, Result};
use tray_icon::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};

use crate::state::AppState;

/// アイコン PNG をバイナリに埋め込む。
/// リポジトリの assets/icon.png がビルド時に読み込まれる。
const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

/// トレイアイコンとそのメニューを保持する構造体。
///
/// `TrayIcon` は drop されるとトレイから消えるため、main で生存させ続ける必要がある。
/// `MenuItem` のハンドルも、イベント受信時の ID 照合に使うため保持する。
pub struct Tray {
    _tray: TrayIcon,
    pub pause_item: MenuItem,
    pub quit_item: MenuItem,
}

impl Tray {
    pub fn new(state: &AppState) -> Result<Self> {
        // アイコン読み込み。
        let icon = load_icon().context("failed to load tray icon")?;

        // メニュー項目を作成。
        let pause_label = if state.is_paused() { "Resume" } else { "Pause" };
        let pause_item = MenuItem::new(pause_label, true, None);
        let quit_item = MenuItem::new("Quit", true, None);

        // メニュー組み立て。
        let menu = Menu::new();
        menu.append(&pause_item)
            .context("failed to append pause item")?;
        menu.append(&PredefinedMenuItem::separator())
            .context("failed to append separator")?;
        menu.append(&quit_item)
            .context("failed to append quit item")?;

        // トレイアイコン本体。
        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Clip Translator")
            .with_icon(icon)
            .build()
            .context("failed to build tray icon")?;

        Ok(Self {
            _tray: tray,
            pause_item,
            quit_item,
        })
    }
}

/// メニューイベントの種類。
pub enum TrayAction {
    TogglePause,
    Quit,
    Other,
}

impl Tray {
    /// 受信した MenuEvent をこのトレイのアクション種別に変換する。
    pub fn classify(&self, event: &MenuEvent) -> TrayAction {
        if event.id == self.pause_item.id() {
            TrayAction::TogglePause
        } else if event.id == self.quit_item.id() {
            TrayAction::Quit
        } else {
            TrayAction::Other
        }
    }

    /// pause 状態に応じてメニュー項目のラベルを更新する。
    pub fn update_pause_label(&self, paused: bool) {
        let label = if paused { "Resume" } else { "Pause" };
        self.pause_item.set_text(label);
    }
}

/// 埋め込んだ PNG バイナリを `Icon` にデコードする。
fn load_icon() -> Result<Icon> {
    // image クレートが入っていないので、tray-icon に同梱されている from_rgba を使う。
    // PNG をデコードするために image クレートを使うか、png クレート単体を使うのが定番。
    // ここでは tray-icon が依存している png デコーダ経由で済ませる。

    let img = image::load_from_memory(ICON_BYTES)
        .context("failed to decode icon PNG")?
        .to_rgba8();
    let (w, h) = img.dimensions();
    let rgba = img.into_raw();

    Icon::from_rgba(rgba, w, h).context("failed to construct Icon from RGBA")
}
