use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait AIProvider {
    async fn generate_commit_message(&self, git_diff: &str) -> Result<String>;
    async fn list_models(&self) -> Result<Vec<String>>;
}

pub struct OpenAIProvider {
    api_key: String,
    model: String,
}

pub struct OpenRouterProvider {
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

impl OpenRouterProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[derive(Serialize)]
struct AIRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct AIResponse {
    choices: Vec<AIChoice>,
}

#[derive(Deserialize)]
struct AIChoice {
    text: String,
}

#[async_trait::async_trait]
impl AIProvider for OpenAIProvider {
    async fn generate_commit_message(&self, git_diff: &str) -> Result<String> {
        let client = Client::new();
        let request_body = AIRequest {
            model: self.model.clone(),
            prompt: format!(
                "Generate a conventional commit message for the following git diff:\n{}",
                git_diff
            ),
            max_tokens: 50,
        };

        let response = client
            .post("https://api.openai.com/v1/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI API request failed: {}", response.status()));
        }

        let response: AIResponse = response.json().await?;

        response.choices.first()
            .map(|choice| choice.text.trim().to_string())
            .ok_or_else(|| anyhow::anyhow!("No commit message generated"))
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        let client = Client::new();
        let response = client
            .get("https://api.openai.com/v1/models")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenAI API request failed: {}", response.status()));
        }

        let response: Value = response.json().await?;
        let models = response["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
            .iter()
            .filter_map(|model| model["id"].as_str().map(String::from))
            .collect();

        Ok(models)
    }
}

#[async_trait::async_trait]
impl AIProvider for OpenRouterProvider {
    async fn generate_commit_message(&self, git_diff: &str) -> Result<String> {
        let client = Client::new();
        let request_body = AIRequest {
            model: self.model.clone(),
            prompt: format!(
                "Generate a conventional commit message for the following git diff:\n{}",
                git_diff
            ),
            max_tokens: 50,
        };

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/your-repo/ai-commit")
            .header("X-Title", "AI Commit Generator")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenRouter API request failed: {}", response.status()));
        }

        let response: AIResponse = response.json().await?;

        response.choices.first()
            .map(|choice| choice.text.trim().to_string())
            .ok_or_else(|| anyhow::anyhow!("No commit message generated"))
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        let client = Client::new();
        let response = client
            .get("https://openrouter.ai/api/v1/models")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://github.com/your-repo/ai-commit")
            .header("X-Title", "AI Commit Generator")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("OpenRouter API request failed: {}", response.status()));
        }

        let response: Value = response.json().await?;
        let models = response["data"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?
            .iter()
            .filter_map(|model| model["id"].as_str().map(String::from))
            .collect();

        Ok(models)
    }
}
