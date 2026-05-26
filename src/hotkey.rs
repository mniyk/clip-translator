// src/hotkey.rs

use anyhow::{Context, Result};
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};

use crate::config::TOGGLE_PAUSE_HOTKEY;

pub struct Hotkey {
    _manager: GlobalHotKeyManager,
    pub toggle_pause_id: u32,
}

impl Hotkey {
    pub fn new() -> Result<Self> {
        let manager = GlobalHotKeyManager::new().context("failed to create hotkey manager")?;

        let hotkey: HotKey = TOGGLE_PAUSE_HOTKEY
            .parse()
            .with_context(|| format!("failed to parse hotkey: {}", TOGGLE_PAUSE_HOTKEY))?;

        let toggle_pause_id = hotkey.id();

        manager
            .register(hotkey)
            .context("failed to register hotkey")?;

        Ok(Self {
            _manager: manager,
            toggle_pause_id,
        })
    }
}

pub enum HotkeyAction {
    TogglePause,
    Other,
}

impl Hotkey {
    pub fn classify(&self, event: &GlobalHotKeyEvent) -> HotkeyAction {
        if event.id == self.toggle_pause_id {
            HotkeyAction::TogglePause
        } else {
            HotkeyAction::Other
        }
    }
}
