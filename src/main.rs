use anyhow::Result;
use clap::{Arg, Command};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Read};

#[derive(Serialize)]
struct ChatGPTRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<ChatGPTChoice>,
}

#[derive(Deserialize)]
struct ChatGPTChoice {
    text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("ai-commit")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates a commit message using ChatGPT")
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Sets the ChatGPT model"),
        )
        .arg(
            Arg::new("api-key")
                .short('k')
                .long("api-key")
                .value_name("API_KEY")
                .help("Sets the ChatGPT API key"),
        )
        .get_matches();

    let model = matches
        .get_one::<String>("model")
        .cloned()
        .unwrap_or_else(|| {
            env::var("CHATGPT_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string())
        });
    let api_key = matches
        .get_one::<String>("api-key")
        .cloned()
        .unwrap_or_else(|| env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY must be set"));

    let mut git_diff = String::new();
    io::stdin().read_to_string(&mut git_diff)?;

    let client = Client::new();
    let request_body = ChatGPTRequest {
        model: model.to_string(),
        prompt: format!(
            "Generate a conventional commit message for the following git diff:\n{}",
            git_diff
        ),
        max_tokens: 50,
    };

    let response = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?
        .json::<ChatGPTResponse>()
        .await?;

    if let Some(commit_message) = response.choices.first() {
        println!("{}", commit_message.text);
    } else {
        eprintln!("Failed to generate commit message");
    }

    Ok(())
}
