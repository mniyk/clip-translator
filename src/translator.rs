// src/translator.rs

use anyhow::{Context, Result};
use rig_core::client::{CompletionClient, Nothing};
use rig_core::completion::Prompt;
use rig_core::providers::ollama;

use crate::config::CliArgs;

const PREAMBLE: &str = "\
You are a translator between English and Japanese.

Rules:
- If the input is English, translate it to natural Japanese.
- If the input is Japanese, translate it to natural English.
- If the input is not natural English or Japanese (URL, code, file path, \
random symbols, mixed gibberish, single word that is a brand/proper noun \
not worth translating, etc.), output exactly: SKIP
- Output ONLY the translated text, or the single word SKIP.
- Do NOT add explanations, quotes, prefixes, or any other text.
- Do NOT wrap the output in quotes or code blocks.
";

const SKIP_MARKER: &str = "SKIP";

pub struct Translator {
    agent: rig_core::agent::Agent<ollama::CompletionModel>,
}

impl Translator {
    pub fn new(args: &CliArgs) -> Result<Self> {
        let client = ollama::Client::builder()
            .api_key(Nothing)
            .base_url(&args.endpoint)
            .build()
            .context("failed to build Ollama client")?;

        let agent = client.agent(&args.model).preamble(PREAMBLE).build();

        Ok(Self { agent })
    }

    pub async fn translate(&self, text: &str) -> Result<Option<String>> {
        let raw = self
            .agent
            .prompt(text)
            .await
            .context("Ollama prompt failed")?;

        let trimmed = raw.trim();
        if trimmed == SKIP_MARKER {
            return Ok(None);
        }

        Ok(Some(trimmed.to_owned()))
    }
}
