// =============================================================================
// LLM Client — v5-legal
// =============================================================================
// SUB-PROJECT A TARGET: This module currently contains the v4.x OpenAI
// implementation adapted for local inference. The full Ollama swap (configurable
// endpoint, model selection, zero-cost tracking) will be implemented in
// Sub-project A.
//
// Current state: Endpoint and model updated to Ollama defaults. API key check
// removed (local inference requires no auth). Cost tracking zeroed (local
// inference has no per-token cost).
// =============================================================================

use crate::core::errors::{CosynError, CosynResult};
use crate::core::types::DraftOutput;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const SYSTEM_PROMPT: &str =
    "You are a controlled drafting engine. Produce a concise, direct response to the user input. Do not include placeholders, filler, or speculative content.";

// TODO(sub-project-a): Make model and endpoint configurable via config file or env var
const MODEL: &str = "qwen2.5:32b"; // Placeholder — final model selected during Sub-project A.1
const MAX_TOKENS: u32 = 1024;
const TEMPERATURE: f64 = 0.3;
const TIMEOUT_SECS: u64 = 60; // Local inference may be slower than cloud API
const ENDPOINT: &str = "http://localhost:11434/v1/chat/completions";

static REQ_COUNT: AtomicU64 = AtomicU64::new(0);
static TOTAL_TOKENS: AtomicU64 = AtomicU64::new(0);

fn estimate_tokens(text: &str) -> u64 {
    // ~4 chars per token is a standard rough estimate
    (text.len() as u64 + 3) / 4
}

#[derive(Serialize)]
struct ChatRequest {
    model: &'static str,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f64,
}

#[derive(Serialize)]
struct Message {
    role: &'static str,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    #[serde(default)]
    content: Option<String>,
}

pub fn draft(input: &str) -> CosynResult<DraftOutput> {
    let body = ChatRequest {
        model: MODEL,
        messages: vec![
            Message {
                role: "system",
                content: SYSTEM_PROMPT.into(),
            },
            Message {
                role: "user",
                content: input.into(),
            },
        ],
        max_tokens: MAX_TOKENS,
        temperature: TEMPERATURE,
    };

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| CosynError::Draft(format!("HTTP client error: {}", e)))?;

    let resp = client
        .post(ENDPOINT)
        .json(&body)
        .send()
        .map_err(|e| CosynError::Draft(format!("Local LLM request failed: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        return Err(CosynError::Draft(format!("Local LLM error {}: {}", status, text)));
    }

    let chat: ChatResponse = resp
        .json()
        .map_err(|e| CosynError::Draft(format!("response parse error: {}", e)))?;

    let text = chat
        .choices
        .first()
        .and_then(|c| c.message.content.as_deref())
        .unwrap_or("")
        .to_string();

    if text.is_empty() {
        return Err(CosynError::Draft("Local LLM returned empty content".into()));
    }

    // Usage telemetry (local inference — zero marginal cost)
    let input_tokens = estimate_tokens(SYSTEM_PROMPT) + estimate_tokens(input);
    let output_tokens = estimate_tokens(&text);
    let call_tokens = input_tokens + output_tokens;
    TOTAL_TOKENS.fetch_add(call_tokens, Ordering::Relaxed);
    let reqs = REQ_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let total_tok = TOTAL_TOKENS.load(Ordering::Relaxed);
    println!(
        "  USAGE → req_count: {} | est_tokens: {} | cost: $0 (local inference)",
        reqs, total_tok
    );

    Ok(DraftOutput { text })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // requires local Ollama server running — run with: cargo test -- --ignored
    fn local_llm_returns_draft() {
        let result = draft("What is 2 + 2?");
        assert!(result.is_ok(), "Local LLM call failed: {:?}", result.err());
        let output = result.unwrap();
        assert!(!output.text.is_empty());
    }
}
