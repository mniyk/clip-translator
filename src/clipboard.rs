// src/clipboard.rs

use anyhow::{Context, Result};
use arboard::Clipboard;

use crate::config::{MAX_CHARS, MIN_CHARS};

pub struct ClipboardWatcher {
    clipboard: Clipboard,
    last_seen: Option<String>,
}

impl ClipboardWatcher {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new().context("failed to access system clipboard")?;
        Ok(Self {
            clipboard,
            last_seen: None,
        })
    }

    pub fn poll(&mut self) -> Option<String> {
        let text = self.clipboard.get_text().ok()?;

        let trimmed = text.trim();
        let len = trimmed.chars().count();
        if len < MIN_CHARS || len > MAX_CHARS {
            return None;
        }

        if self.last_seen.as_deref() == Some(trimmed) {
            return None;
        }

        let owned = trimmed.to_owned();
        self.last_seen = Some(owned.clone());
        Some(owned)
    }
}
