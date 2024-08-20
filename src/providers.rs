use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[async_trait]
pub trait AIProvider {
    async fn generate_commit_message(&self, git_diff: &str) -> Result<String>;
    async fn list_models(&self) -> Result<Vec<String>>;
}

pub struct BaseProvider {
    api_key: String,
    model: String,
    client: Client,
    base_url: String,
}

impl BaseProvider {
    pub fn new(api_key: String, model: String, base_url: String) -> Self {
        Self {
            api_key,
            model,
            client: Client::new(),
            base_url,
        }
    }

    #[cfg(test)]
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }
}

pub struct OpenAIProvider(pub BaseProvider);
pub struct OpenRouterProvider(pub BaseProvider);

impl OpenAIProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self(BaseProvider::new(
            api_key,
            model,
            "https://api.openai.com".to_string(),
        ))
    }
}

impl OpenRouterProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self(BaseProvider::new(
            api_key,
            model,
            "https://openrouter.ai/api".to_string(),
        ))
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
        let request_body = AIRequest {
            model: self.0.model.clone(),
            prompt: format!(
                "Generate a concise conventional commit message for the following git diff. Provide only the commit message, without any additional text or formatting:\n{}",
                git_diff
            ),
            max_tokens: 50,
        };

        let response = self
            .0
            .client
            .post(format!("{}/v1/completions", self.0.base_url))
            .header("Authorization", format!("Bearer {}", self.0.api_key))
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "OpenAI API request failed: {}",
                response.status()
            ));
        }

        let response: AIResponse = response.json().await?;

        response
            .choices
            .first()
            .map(|choice| choice.text.trim().to_string())
            .ok_or_else(|| anyhow::anyhow!("No commit message generated"))
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        let response = self
            .0
            .client
            .get(format!("{}/v1/models", self.0.base_url))
            .header("Authorization", format!("Bearer {}", self.0.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "OpenAI API request failed: {}",
                response.status()
            ));
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
        let request_body = AIRequest {
            model: self.0.model.clone(),
            prompt: format!(
                "Generate a concise conventional commit message for the following git diff. Provide only the commit message, without any additional text or formatting:\n{}",
                git_diff
            ),
            max_tokens: 50,
        };

        let response = self
            .0
            .client
            .post(format!("{}/v1/chat/completions", self.0.base_url))
            .header("Authorization", format!("Bearer {}", self.0.api_key))
            .header("HTTP-Referer", "https://github.com/EgorHenek/ai-commit")
            .header("X-Title", "AI Commit Generator")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "OpenRouter API request failed: {}",
                response.status()
            ));
        }

        let response: AIResponse = response.json().await?;

        response
            .choices
            .first()
            .map(|choice| choice.text.trim().to_string())
            .ok_or_else(|| anyhow::anyhow!("No commit message generated"))
    }

    async fn list_models(&self) -> Result<Vec<String>> {
        let response = self
            .0
            .client
            .get(format!("{}/v1/models", self.0.base_url))
            .header("Authorization", format!("Bearer {}", self.0.api_key))
            .header("HTTP-Referer", "https://github.com/EgorHenek/ai-commit")
            .header("X-Title", "AI Commit Generator")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "OpenRouter API request failed: {}",
                response.status()
            ));
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
