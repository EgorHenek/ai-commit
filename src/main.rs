mod providers;
#[cfg(test)]
mod providers_test;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, Arg, Command};

use providers::{AIProvider, OpenAIProvider, OpenRouterProvider};
use std::env;
use std::io::{self, Read};
use strum::{Display, EnumString};

#[derive(Debug, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
enum ProviderType {
    OpenAI,
    OpenRouter,
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("ai-commit")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .value_name("MODEL")
                .help("Sets the AI model (default: gpt-4o-mini)"),
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
        .arg(
            Arg::new("list-models")
                .long("list-models")
                .help("Lists available models for the selected provider")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let model = matches
        .get_one::<String>("model")
        .cloned()
        .unwrap_or_else(|| env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string()));

    let provider_type: ProviderType = matches
        .get_one::<String>("provider")
        .unwrap()
        .parse()
        .expect("Invalid provider type");

    let api_key = matches
        .get_one::<String>("api-key")
        .cloned()
        .unwrap_or_else(|| match provider_type {
            ProviderType::OpenAI => env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            ProviderType::OpenRouter => {
                env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY must be set")
            }
        });

    if matches.get_flag("list-models") {
        let models = match provider_type {
            ProviderType::OpenAI => {
                let provider = OpenAIProvider::new(api_key, model);
                provider.list_models().await?
            }
            ProviderType::OpenRouter => {
                let provider = OpenRouterProvider::new(api_key, model);
                provider.list_models().await?
            }
        };
        println!("Available models for {}:", provider_type);
        for model in models {
            println!("- {}", model);
        }
    } else {
        let mut git_diff = String::new();
        io::stdin().read_to_string(&mut git_diff)?;

        let commit_message = match provider_type {
            ProviderType::OpenAI => {
                let provider = OpenAIProvider::new(api_key, model);
                provider.generate_commit_message(&git_diff).await?
            }
            ProviderType::OpenRouter => {
                let provider = OpenRouterProvider::new(api_key, model);
                provider.generate_commit_message(&git_diff).await?
            }
        };
        println!("{}", commit_message);
    }

    Ok(())
}
