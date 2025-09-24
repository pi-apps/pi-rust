use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub base_url: Url,
    pub timeout: Duration,
    pub retry_config: RetryConfig,
    pub user_agent: String,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f64,
}

impl ClientConfig {
    pub fn new(api_key: String) -> crate::Result<Self> {
        if api_key.is_empty() {
            return Err(crate::PiError::Configuration("API key cannot be empty".to_string()));
        }

        Ok(Self {
            api_key,
            base_url: Url::parse("https://api.minepi.com/v2").unwrap(),
            timeout: Duration::from_secs(30),
            retry_config: RetryConfig::default(),
            user_agent: format!("pi-rust/{}", env!("CARGO_PKG_VERSION")),
        })
    }

    pub fn builder(api_key: String) -> ClientConfigBuilder {
        ClientConfigBuilder::new(api_key)
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_factor: 2.0,
        }
    }
}

pub struct ClientConfigBuilder {
    config: ClientConfig,
}

impl ClientConfigBuilder {
    pub fn new(api_key: String) -> Self {
        Self {
            config: ClientConfig::new(api_key).expect("Invalid API key"),
        }
    }

    pub fn base_url(mut self, url: Url) -> Self {
        self.config.base_url = url;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn build(self) -> ClientConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = ClientConfig::new("test-key".to_string()).unwrap();
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_empty_api_key_fails() {
        let result = ClientConfig::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let config = ClientConfig::builder("test-key".to_string())
            .timeout(Duration::from_secs(60))
            .build();
        assert_eq!(config.timeout, Duration::from_secs(60));
    }
}