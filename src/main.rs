use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ----------------------------- CLI -----------------------------

#[derive(Parser)]
#[command(name="rag-rs", about="Chat or RAG. Use `chat` for a general LLM answer (no files).")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// General LLM chat (no files/RAG). Example:
    /// cargo run -- chat "Explain Rust ownership simply"
    Chat {
        /// Your prompt/question
        prompt: String,
        /// Optional system instruction (role)
        #[arg(long, default_value = "You are a concise, helpful assistant.")]
        system: String,
        /// Model name (OpenAI)
        #[arg(long, default_value = "gpt-4o-mini")]
        model: String,
        /// Temperature (0 = deterministic)
        #[arg(long, default_value_t = 0.2)]
        temperature: f32,
    },

    // The old RAG commands can stay if you want to keep them. You can remove them if not needed.
    /// (Optional) Existing RAG commands — ignore if you just want chat
    #[allow(dead_code)]
    Dummy,
}

// ----------------------- OpenAI Chat ---------------------------

#[derive(Serialize)]
struct ChatReq<'a> {
    model: &'a str,
    messages: Vec<ChatMsg<'a>>,
    temperature: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct ChatMsg<'a> {
    role: &'a str,
    content: String,
}

#[derive(Deserialize)]
struct ChatResp {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMsgOwned,
}

#[derive(Deserialize)]
struct ChatMsgOwned {
    role: String,
    content: String,
}

fn openai_http() -> Result<reqwest::blocking::Client> {
    Ok(reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?)
}

fn chat_openai(model: &str, system: &str, user: &str, temperature: f32) -> Result<String> {
    let key = std::env::var("OPENAI_API_KEY")
        .context("Set OPENAI_API_KEY env var (export OPENAI_API_KEY=sk-...)")?;
    let client = openai_http()?;

    let req = ChatReq {
        model,
        messages: vec![
            ChatMsg { role: "system", content: system.to_string() },
            ChatMsg { role: "user", content: user.to_string() },
        ],
        temperature,
    };

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(key)
        .json(&req)
        .send()?
        .error_for_status()?;

    let parsed: ChatResp = resp.json()?;
    let out = parsed
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .unwrap_or_else(|| "(no output)".to_string());
    Ok(out)
}

// ----------------------------- main ---------------------------

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Chat { prompt, system, model, temperature } => {
            let answer = chat_openai(&model, &system, &prompt, temperature)?;
            println!("\n=== Answer ===\n{}\n", answer);
        }
        Cmd::Dummy => {
            println!("This is a placeholder for old RAG commands. Use `chat` instead.");
        }
    }

    Ok(())
}
