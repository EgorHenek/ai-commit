mod providers;

use anyhow::Result;
use clap::{Arg, Command};
use providers::{AIProvider, OpenAIProvider, OpenRouterProvider};
use std::env;
use std::io::{self, Read};
use strum::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
enum ProviderType {
    OpenAI,
    OpenRouter,
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("ai-commit")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates a commit message using AI")
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Sets the AI model"),
        )
        .arg(
            Arg::new("api-key")
                .short('k')
                .long("api-key")
                .value_name("API_KEY")
                .help("Sets the API key"),
        )
        .arg(
            Arg::new("provider")
                .short('p')
                .long("provider")
                .value_name("PROVIDER")
                .help("Sets the AI provider (openai or openrouter)")
                .default_value("openai"),
        )
        .get_matches();

    let model = matches
        .get_one::<String>("model")
        .cloned()
        .unwrap_or_else(|| env::var("AI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string()));

    let api_key = matches
        .get_one::<String>("api-key")
        .cloned()
        .unwrap_or_else(|| env::var("AI_API_KEY").expect("AI_API_KEY must be set"));

    let provider_type: ProviderType = matches
        .get_one::<String>("provider")
        .unwrap()
        .parse()
        .expect("Invalid provider type");

    let mut git_diff = String::new();
    io::stdin().read_to_string(&mut git_diff)?;

    let provider: Box<dyn AIProvider> = match provider_type {
        ProviderType::OpenAI => Box::new(OpenAIProvider::new(api_key, model)),
        ProviderType::OpenRouter => Box::new(OpenRouterProvider::new(api_key, model)),
    };

    let commit_message = provider.generate_commit_message(&git_diff).await?;
    println!("{}", commit_message);

    Ok(())
}
