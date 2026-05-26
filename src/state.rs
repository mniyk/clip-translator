// src/state.rs

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// アプリ全体で共有する状態。
///
/// 現状は「一時停止フラグ」のみ。tray/hotkey/clipboard ループ間で共有される。
/// `Clone` は `Arc` の中身を共有する。
#[derive(Clone)]
pub struct AppState {
    paused: Arc<AtomicBool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            paused: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 現在 pause 中か?
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    /// pause 状態をトグルし、トグル後の状態を返す。
    /// 返り値の `true` は「pause になった」、`false` は「resume した」。
    pub fn toggle(&self) -> bool {
        // 現在値を読んで反転させる。複数スレッドから同時にトグルされる可能性は低いが、
        // fetch_xor を使って原子的に反転させる。
        let prev = self.paused.fetch_xor(true, Ordering::Relaxed);
        !prev
    }
}