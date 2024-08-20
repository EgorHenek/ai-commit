#[cfg(test)]
mod tests {
    use super::super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_openai_generate_commit_message() {
        let mut server = mockito::Server::new();
        let mock = server.mock("POST", "/v1/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"text":"feat: Add new feature"}]}"#)
            .create();

        let provider = OpenAIProvider::new("test_key".to_string(), "test_model".to_string());
        let result = provider.generate_commit_message("test diff").await;

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat: Add new feature");
    }

    #[tokio::test]
    async fn test_openrouter_generate_commit_message() {
        let mut server = mockito::Server::new();
        let mock = server.mock("POST", "/api/v1/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"text":"fix: Resolve bug"}]}"#)
            .create();

        let provider = OpenRouterProvider::new("test_key".to_string(), "test_model".to_string());
        let result = provider.generate_commit_message("test diff").await;

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "fix: Resolve bug");
    }

    #[tokio::test]
    async fn test_openai_list_models() {
        let mut server = mockito::Server::new();
        let mock = server.mock("GET", "/v1/models")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data":[{"id":"model1"},{"id":"model2"}]}"#)
            .create();

        let provider = OpenAIProvider::new("test_key".to_string(), "test_model".to_string());
        let result = provider.list_models().await;

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["model1".to_string(), "model2".to_string()]);
    }

    #[tokio::test]
    async fn test_openrouter_list_models() {
        let mut server = mockito::Server::new();
        let mock = server.mock("GET", "/api/v1/models")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"data":[{"id":"model3"},{"id":"model4"}]}"#)
            .create();

        let provider = OpenRouterProvider::new("test_key".to_string(), "test_model".to_string());
        let result = provider.list_models().await;

        mock.assert();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["model3".to_string(), "model4".to_string()]);
    }
}
