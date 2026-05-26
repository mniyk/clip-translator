// src/notifier.rs

use anyhow::{Context, Result};
use notify_rust::Notification;

use crate::config::{NOTIFICATION_TIMEOUT_MS, NOTIFICATION_TITLE};

const APP_ID: &str = "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

pub fn notify(body: &str) -> Result<()> {
    Notification::new()
        .summary(NOTIFICATION_TITLE)
        .body(body)
        .app_id(APP_ID)
        .timeout(notify_rust::Timeout::Milliseconds(NOTIFICATION_TIMEOUT_MS))
        .show()
        .context("failed to show notification")?;
    Ok(())
}
