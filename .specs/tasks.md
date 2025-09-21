# Implementation Plan

- [ ] 1. Set up project foundation and basic structure

  - **Initialize Cargo project**: Run `cargo init --lib pi-rust` and configure Cargo.toml with:

    ```toml
    [package]
    name = "pi-rust"
    version = "0.1.0"
    edition = "2021"
    authors = ["Your Name <email@example.com>"]
    description = "Pi Network Rust SDK for server-side applications"
    license = "MIT OR Apache-2.0"
    repository = "https://github.com/username/pi-rust"
    keywords = ["pi-network", "blockchain", "stellar", "payments"]
    categories = ["api-bindings", "cryptocurrency"]

    [dependencies]
    tokio = { version = "1.0", features = ["full"] }
    reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    thiserror = "1.0"
    url = "2.0"
    chrono = { version = "0.4", features = ["serde"] }
    uuid = { version = "1.0", features = ["v4", "serde"] }

    [dev-dependencies]
    wiremock = "0.5"
    criterion = "0.5"
    tokio-test = "0.4"
    ```

  - **Create module structure**: Create these files with basic module declarations:
    - `src/lib.rs` - Main library entry with `pub mod` declarations
    - `src/client.rs` - Core client struct (empty for now)
    - `src/config.rs` - Configuration types (empty for now)
    - `src/errors.rs` - Error types (empty for now)
    - `src/models/mod.rs` - Data models module
    - `src/payments/mod.rs` - Payment operations module
    - `src/auth/mod.rs` - Authentication module
    - `src/stellar/mod.rs` - Stellar operations module
  - **Set up GitHub Actions**: Create `.github/workflows/ci.yml` with basic Rust CI pipeline:
    ```yaml
    name: CI
    on: [push, pull_request]
    jobs:
      test:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v3
          - uses: actions-rs/toolchain@v1
            with:
              toolchain: stable
          - run: cargo build
          - run: cargo test
          - run: cargo clippy -- -D warnings
          - run: cargo fmt -- --check
    ```
  - _Requirements: 7.1, 7.2, 11.1, 11.2, 11.3_

- [ ] 2. Implement core error handling and configuration

  - **Create error types in `src/errors.rs`**:

    ```rust
    use std::time::Duration;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum PiError {
        #[error("HTTP request failed: {0}")]
        Http(#[from] reqwest::Error),

        #[error("JSON serialization failed: {0}")]
        Json(#[from] serde_json::Error),

        #[error("Pi Network API error: {error_name} - {error_message}")]
        PiNetwork {
            error_name: String,
            error_message: String,
            payment: Option<crate::models::PaymentDto>,
        },

        #[error("Authentication failed: {0}")]
        Authentication(String),

        #[error("Invalid configuration: {0}")]
        Configuration(String),

        #[error("Stellar operation failed: {0}")]
        Stellar(String),

        #[error("Insufficient balance: available {available}, required {required}")]
        InsufficientBalance { available: f64, required: f64 },

        #[error("Timeout occurred after {duration:?}")]
        Timeout { duration: Duration },
    }

    pub type Result<T> = std::result::Result<T, PiError>;
    ```

  - **Create configuration in `src/config.rs`**:

    ```rust
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
    ```

  - **Write tests in `src/config.rs`**:

    ```rust
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
    ```

  - _Requirements: 1.4, 1.5, 9.1, 9.2, 9.3, 9.4_

- [ ] 3. Create data models and serialization

  - **Create `src/models/mod.rs`** with module declarations:

    ```rust
    pub mod auth;
    pub mod payment;
    pub mod stellar;
    pub mod common;

    pub use auth::*;
    pub use payment::*;
    pub use stellar::*;
    pub use common::*;
    ```

  - **Implement auth models in `src/models/auth.rs`**:

    ```rust
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserDto {
        pub uid: String,
        pub username: String,
    }

    // Note: The access token is provided by the client when making requests,
    // not returned in the UserDTO response from the API
    ```

  - **Implement payment models in `src/models/payment.rs`**:

    ```rust
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PaymentDto {
        pub identifier: String,
        #[serde(rename = "Pioneer_uid")]
        pub pioneer_uid: String,
        pub amount: f64,
        pub memo: String,
        pub metadata: serde_json::Value,
        pub to_address: String,
        pub created_at: String,
        pub status: PaymentStatus,
        pub transaction: Option<TransactionData>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PaymentStatus {
        pub developer_approved: bool,
        pub transaction_verified: bool,
        pub developer_completed: bool,
        pub canceled: bool,
        #[serde(rename = "Pioneer_cancelled")]
        pub pioneer_cancelled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TransactionData {
        pub txid: String,
        pub verified: bool,
        #[serde(rename = "_link")]
        pub link: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CompletePaymentRequest {
        pub txid: String,
    }

    // Note: Payment creation is handled client-side via Pi App Platform SDK,
    // not through server APIs, so CreatePaymentRequest is not needed
    ```

  - **Implement Stellar models in `src/models/stellar.rs`**:

    ```rust
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Network {
        PiMainnet,
        PiTestnet,
        StellarTestnet,
    }

    impl Network {
        pub fn server_url(&self) -> &'static str {
            match self {
                Network::PiMainnet => "https://api.mainnet.minepi.com",
                Network::PiTestnet => "https://api.testnet.minepi.com",
                Network::StellarTestnet => "https://horizon-testnet.stellar.org",
            }
        }

        pub fn network_passphrase(&self) -> &'static str {
            match self {
                Network::PiMainnet => "Pi Network",
                Network::PiTestnet => "Pi Testnet",
                Network::StellarTestnet => "Test SDF Network ; September 2015",
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct SendAssetsParams {
        pub network: Network,
        pub source_secret: String,
        pub destination: String,
        pub amount: f64,
        pub memo: Option<String>,
        pub fee: Option<u32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TransactionResponse {
        pub hash: String,
        pub ledger: u32,
        pub envelope_xdr: String,
        pub result_xdr: String,
        pub result_meta_xdr: String,
    }
    ```

  - **Add common models in `src/models/common.rs`**:

    ```rust
    use serde::{Deserialize, Serialize};
    use crate::models::PaymentDto;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PiNetworkError {
        pub error: String,
        pub error_message: String,
        pub payment: Option<PaymentDto>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TransactionId {
        #[serde(rename = "txid")]
        pub tx_id: String,
    }
    ```

  - **Write serialization tests in each model file**:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_user_profile_serialization() {
            let profile = UserProfile {
                access_token: "token123".to_string(),
                user: User {
                    uid: "user123".to_string(),
                    username: "testuser".to_string(),
                    credentials: Credentials {
                        scopes: vec!["payments".to_string()],
                        valid_until: ValidTime {
                            timestamp: 1234567890,
                            iso8601: chrono::Utc::now(),
                        },
                    },
                },
            };

            let json = serde_json::to_string(&profile).unwrap();
            let deserialized: UserProfile = serde_json::from_str(&json).unwrap();
            assert_eq!(profile.access_token, deserialized.access_token);
        }
    }
    ```

  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 4. Build HTTP client foundation

  - **Create core client in `src/client.rs`**:

    ```rust
    use reqwest::{Client, RequestBuilder, Response};
    use std::time::Duration;
    use url::Url;
    use crate::{config::ClientConfig, errors::PiError, Result};

    #[derive(Debug, Clone)]
    pub struct PiNetworkClient {
        http_client: Client,
        config: ClientConfig,
    }

    impl PiNetworkClient {
        pub fn new(api_key: String) -> Result<Self> {
            let config = ClientConfig::new(api_key)?;
            Self::with_config(config)
        }

        pub fn with_config(config: ClientConfig) -> Result<Self> {
            let http_client = Client::builder()
                .timeout(config.timeout)
                .user_agent(&config.user_agent)
                .build()
                .map_err(PiError::Http)?;

            Ok(Self {
                http_client,
                config,
            })
        }

        pub(crate) fn get(&self, path: &str) -> RequestBuilder {
            let url = self.config.base_url.join(path).unwrap();
            self.http_client
                .get(url)
                .header("Accept", "application/json")
        }

        pub(crate) fn post(&self, path: &str) -> RequestBuilder {
            let url = self.config.base_url.join(path).unwrap();
            self.http_client
                .post(url)
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
        }

        pub(crate) fn with_api_key_auth(&self, request: RequestBuilder) -> RequestBuilder {
            request.header("Authorization", format!("Key {}", self.config.api_key))
        }

        pub(crate) fn with_bearer_auth(&self, request: RequestBuilder, token: &str) -> RequestBuilder {
            request.header("Authorization", format!("Bearer {}", token))
        }

        pub(crate) async fn execute_request<T>(&self, request: RequestBuilder) -> Result<T>
        where
            T: serde::de::DeserializeOwned,
        {
            let response = self.execute_with_retry(request).await?;
            self.handle_response(response).await
        }

        async fn execute_with_retry(&self, mut request: RequestBuilder) -> Result<Response> {
            let mut attempts = 0;
            let max_attempts = self.config.retry_config.max_retries + 1;

            loop {
                let req = request.try_clone()
                    .ok_or_else(|| PiError::Configuration("Request cannot be cloned".to_string()))?;

                match req.send().await {
                    Ok(response) => return Ok(response),
                    Err(e) if attempts < max_attempts - 1 && e.is_timeout() => {
                        attempts += 1;
                        let delay = self.calculate_retry_delay(attempts);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    Err(e) => return Err(PiError::Http(e)),
                }
            }
        }

        fn calculate_retry_delay(&self, attempt: u32) -> Duration {
            let delay = self.config.retry_config.initial_delay.as_millis() as f64
                * self.config.retry_config.backoff_factor.powi(attempt as i32);

            Duration::from_millis(delay.min(self.config.retry_config.max_delay.as_millis() as f64) as u64)
        }

        async fn handle_response<T>(&self, response: Response) -> Result<T>
        where
            T: serde::de::DeserializeOwned,
        {
            if response.status().is_success() {
                let text = response.text().await.map_err(PiError::Http)?;
                serde_json::from_str(&text).map_err(PiError::Json)
            } else {
                let status = response.status();
                let text = response.text().await.map_err(PiError::Http)?;

                // Try to parse as Pi Network error
                if let Ok(pi_error) = serde_json::from_str::<crate::models::PiNetworkError>(&text) {
                    Err(PiError::PiNetwork {
                        error_name: pi_error.error,
                        error_message: pi_error.error_message,
                        payment: pi_error.payment,
                    })
                } else {
                    Err(PiError::Http(reqwest::Error::from(
                        reqwest::StatusCode::from_u16(status.as_u16()).unwrap()
                    )))
                }
            }
        }
    }
    ```

  - **Write comprehensive tests in `src/client.rs`**:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path, header};

        #[tokio::test]
        async fn test_client_creation() {
            let client = PiNetworkClient::new("test-key".to_string()).unwrap();
            assert_eq!(client.config.api_key, "test-key");
        }

        #[tokio::test]
        async fn test_successful_request() {
            let mock_server = MockServer::start().await;
            let expected_response = serde_json::json!({"status": "success"});

            Mock::given(method("GET"))
                .and(path("/test"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let request = client.get("/test");
            let response: serde_json::Value = client.execute_request(request).await.unwrap();
            assert_eq!(response["status"], "success");
        }

        #[tokio::test]
        async fn test_pi_network_error_handling() {
            let mock_server = MockServer::start().await;
            let error_response = serde_json::json!({
                "error": "PAYMENT_NOT_FOUND",
                "error_message": "Payment with identifier not found"
            });

            Mock::given(method("GET"))
                .and(path("/error"))
                .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let request = client.get("/error");
            let result: Result<serde_json::Value> = client.execute_request(request).await;

            match result {
                Err(PiError::PiNetwork { error_name, error_message, .. }) => {
                    assert_eq!(error_name, "PAYMENT_NOT_FOUND");
                    assert_eq!(error_message, "Payment with identifier not found");
                }
                _ => panic!("Expected PiNetwork error"),
            }
        }

        #[tokio::test]
        async fn test_retry_logic() {
            let mock_server = MockServer::start().await;

            // First request fails, second succeeds
            Mock::given(method("GET"))
                .and(path("/retry"))
                .respond_with(ResponseTemplate::new(500))
                .up_to_n_times(1)
                .mount(&mock_server)
                .await;

            Mock::given(method("GET"))
                .and(path("/retry"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&serde_json::json!({"retry": "success"})))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let request = client.get("/retry");
            let response: serde_json::Value = client.execute_request(request).await.unwrap();
            assert_eq!(response["retry"], "success");
        }
    }
    ```

  - _Requirements: 1.1, 1.2, 1.3, 10.1, 10.2_

- [ ] 5. Implement authentication operations

  - **Create auth module in `src/auth/mod.rs`**:
    ```rust
    pub mod client;
    pub use client::*;
    ```
  - **Implement auth client in `src/auth/client.rs`**:

    ````rust
    use crate::{
        client::PiNetworkClient,
        models::{UserProfile},
        errors::PiError,
        Result,
    };

    impl PiNetworkClient {
        /// Retrieve user information using an access token
        ///
        /// # Arguments
        /// * `access_token` - The user's access token from Pi Network authentication
        ///
        /// # Returns
        /// * `Result<UserDto>` - User information with uid and username
        ///
        /// # Example
        /// ```rust
        /// use pi_rust::PiNetworkClient;
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
        ///     let client = PiNetworkClient::new("your-api-key".to_string())?;
        ///     let user = client.get_user_info("user-access-token").await?;
        ///     println!("User: {} ({})", user.username, user.uid);
        ///     Ok(())
        /// }
        /// ```
        pub async fn get_user_info(&self, access_token: &str) -> Result<UserDto> {
            if access_token.is_empty() {
                return Err(PiError::Authentication("Access token cannot be empty".to_string()));
            }

            let request = self.get("/me");
            let request = self.with_bearer_auth(request, access_token);

            self.execute_request(request).await
                .map_err(|e| match e {
                    PiError::PiNetwork { error_name, error_message, .. }
                        if error_name == "UNAUTHORIZED" || error_name == "INVALID_TOKEN" => {
                        PiError::Authentication(format!("Invalid or expired access token: {}", error_message))
                    }
                    other => other,
                })
        }

        /// Validate if an access token is still valid
        ///
        /// # Arguments
        /// * `access_token` - The access token to validate
        ///
        /// # Returns
        /// * `Result<bool>` - True if token is valid, false otherwise
        pub async fn validate_access_token(&self, access_token: &str) -> Result<bool> {
            match self.get_user_info(access_token).await {
                Ok(_) => Ok(true),
                Err(PiError::Authentication(_)) => Ok(false),
                Err(e) => Err(e),
            }
        }
    }
    ````

  - **Add comprehensive tests in `src/auth/client.rs`**:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::config::ClientConfig;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path, header};
        use chrono::Utc;

        #[tokio::test]
        async fn test_get_user_info_success() {
            let mock_server = MockServer::start().await;
            let expected_user = crate::models::UserDto {
                uid: "user123".to_string(),
                username: "testuser".to_string(),
            };

            Mock::given(method("GET"))
                .and(path("/me"))
                .and(header("authorization", "Bearer valid-token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_user))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let result = client.get_user_info("valid-token").await.unwrap();
            assert_eq!(result.uid, "user123");
            assert_eq!(result.username, "testuser");
        }

        #[tokio::test]
        async fn test_get_user_profile_invalid_token() {
            let mock_server = MockServer::start().await;
            let error_response = serde_json::json!({
                "error": "UNAUTHORIZED",
                "error_message": "Invalid access token"
            });

            Mock::given(method("GET"))
                .and(path("/me"))
                .and(header("authorization", "Bearer invalid-token"))
                .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let result = client.get_user_profile("invalid-token").await;
            match result {
                Err(PiError::Authentication(msg)) => {
                    assert!(msg.contains("Invalid or expired access token"));
                }
                _ => panic!("Expected authentication error"),
            }
        }

        #[tokio::test]
        async fn test_empty_access_token() {
            let client = PiNetworkClient::new("test-key".to_string()).unwrap();
            let result = client.get_user_profile("").await;

            match result {
                Err(PiError::Authentication(msg)) => {
                    assert_eq!(msg, "Access token cannot be empty");
                }
                _ => panic!("Expected authentication error for empty token"),
            }
        }

        #[tokio::test]
        async fn test_validate_access_token() {
            let mock_server = MockServer::start().await;
            let profile = crate::models::UserProfile {
                access_token: "token123".to_string(),
                user: crate::models::User {
                    uid: "user123".to_string(),
                    username: "testuser".to_string(),
                    credentials: crate::models::Credentials {
                        scopes: vec!["payments".to_string()],
                        valid_until: crate::models::ValidTime {
                            timestamp: 1234567890,
                            iso8601: Utc::now(),
                        },
                    },
                },
            };

            Mock::given(method("GET"))
                .and(path("/me"))
                .and(header("authorization", "Bearer valid-token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&profile))
                .mount(&mock_server)
                .await;

            Mock::given(method("GET"))
                .and(path("/me"))
                .and(header("authorization", "Bearer invalid-token"))
                .respond_with(ResponseTemplate::new(401).set_body_json(&serde_json::json!({
                    "error": "UNAUTHORIZED",
                    "error_message": "Invalid token"
                })))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            assert!(client.validate_access_token("valid-token").await.unwrap());
            assert!(!client.validate_access_token("invalid-token").await.unwrap());
        }
    }
    ```

  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [ ] 6. Implement payment management operations

  - **Create payment module in `src/payments/mod.rs`**:
    ```rust
    pub mod client;
    pub use client::*;
    ```
  - **Implement payment client in `src/payments/client.rs`**:

    ````rust
    use crate::{
        client::PiNetworkClient,
        models::{PaymentDto, CompletePaymentRequest},
        errors::PiError,
        Result,
    };

    impl PiNetworkClient {
        /// Retrieve a payment by its payment_id
        ///
        /// # Arguments
        /// * `payment_id` - The payment identifier
        ///
        /// # Returns
        /// * `Result<PaymentDto>` - The payment details
        ///
        /// # Example
        /// ```rust
        /// use pi_rust::PiNetworkClient;
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
        ///     let client = PiNetworkClient::new("your-api-key".to_string())?;
        ///     let payment = client.get_payment("payment_id_here").await?;
        ///     println!("Payment amount: {} Pi", payment.amount);
        ///     Ok(())
        /// }
        /// ```
        pub async fn get_payment(&self, payment_id: &str) -> Result<PaymentDto> {
            if payment_id.is_empty() {
                return Err(PiError::Configuration("Payment ID cannot be empty".to_string()));
            }

            let path = format!("/payments/{}", payment_id);
            let request = self.get(&path);
            let request = self.with_api_key_auth(request);

            self.execute_request(request).await
        }

        /// Approve a payment (developer approval)
        ///
        /// # Arguments
        /// * `payment_id` - The payment identifier to approve
        ///
        /// # Returns
        /// * `Result<PaymentDto>` - The updated payment with approval status
        pub async fn approve_payment(&self, payment_id: &str) -> Result<PaymentDto> {
            if payment_id.is_empty() {
                return Err(PiError::Configuration("Payment ID cannot be empty".to_string()));
            }

            let path = format!("/payments/{}/approve", payment_id);
            let request = self.post(&path);
            let request = self.with_api_key_auth(request);

            self.execute_request(request).await
        }

        /// Complete a payment with transaction ID
        ///
        /// # Arguments
        /// * `payment_id` - The payment identifier to complete
        /// * `tx_id` - The blockchain transaction ID
        ///
        /// # Returns
        /// * `Result<PaymentDto>` - The completed payment
        pub async fn complete_payment(&self, payment_id: &str, tx_id: &str) -> Result<PaymentDto> {
            if payment_id.is_empty() {
                return Err(PiError::Configuration("Payment ID cannot be empty".to_string()));
            }

            if tx_id.is_empty() {
                return Err(PiError::Configuration("Transaction ID cannot be empty".to_string()));
            }

            let path = format!("/payments/{}/complete", payment_id);
            let request_body = CompletePaymentRequest {
                txid: tx_id.to_string(),
            };

            let request = self.post(&path);
            let request = self.with_api_key_auth(request);
            let request = request.json(&request_body);

            self.execute_request(request).await
        }
    }
    ````

  - **Add comprehensive tests in `src/payments/client.rs`**:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::config::ClientConfig;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path, header, body_json};
        use chrono::Utc;
        use serde_json::json;

        fn create_test_payment() -> PaymentDto {
            PaymentDto {
                identifier: "payment123".to_string(),
                pioneer_uid: "user123".to_string(),
                amount: 10.5,
                memo: "Test payment".to_string(),
                metadata: serde_json::json!({"test": true}),
                to_address: "GTEST456".to_string(),
                created_at: "2023-01-01T00:00:00Z".to_string(),
                status: crate::models::PaymentStatus {
                    developer_approved: false,
                    transaction_verified: false,
                    developer_completed: false,
                    canceled: false,
                    pioneer_cancelled: false,
                },
                transaction: None,
            }
        }

        // Note: Payment creation is handled client-side, so we don't test it here
        // Instead, we test the server-side operations: get, approve, and complete

        #[tokio::test]
        async fn test_get_payment_success() {
            let mock_server = MockServer::start().await;
            let expected_payment = create_test_payment();

            Mock::given(method("GET"))
                .and(path("/payments/payment123"))
                .and(header("authorization", "Key test-key"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&expected_payment))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let result = client.get_payment("payment123").await.unwrap();
            assert_eq!(result.identifier, "payment123");
        }

        #[tokio::test]
        async fn test_approve_payment_success() {
            let mock_server = MockServer::start().await;
            let mut approved_payment = create_test_payment();
            approved_payment.status.developer_approved = true;

            Mock::given(method("POST"))
                .and(path("/payments/payment123/approve"))
                .and(header("authorization", "Key test-key"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&approved_payment))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let result = client.approve_payment("payment123").await.unwrap();
            assert!(result.status.developer_approved);
        }

        #[tokio::test]
        async fn test_complete_payment_success() {
            let mock_server = MockServer::start().await;
            let mut completed_payment = create_test_payment();
            completed_payment.status.developer_completed = true;
            completed_payment.transaction = Some(crate::models::TransactionData {
                txid: "tx123".to_string(),
                verified: true,
                link: "https://stellar.expert/explorer/testnet/tx/tx123".to_string(),
            });

            let expected_request = json!({
                "txid": "tx123"
            });

            Mock::given(method("POST"))
                .and(path("/payments/payment123/complete"))
                .and(header("authorization", "Key test-key"))
                .and(body_json(&expected_request))
                .respond_with(ResponseTemplate::new(200).set_body_json(&completed_payment))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let result = client.complete_payment("payment123", "tx123").await.unwrap();
            assert!(result.status.developer_completed);
            assert_eq!(result.transaction.as_ref().unwrap().txid, "tx123");
        }
    }
    ```

  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5, 2.6_

- [ ] 7. Build Stellar blockchain integration

  - **Create stellar module in `src/stellar/mod.rs`**:

    ```rust
    pub mod client;
    pub mod networks;

    pub use client::*;
    pub use networks::*;
    ```

  - **Implement network configurations in `src/stellar/networks.rs`**:

    ```rust
    use crate::models::Network;

    impl Network {
        pub fn server_url(&self) -> &'static str {
            match self {
                Network::PiMainnet => "https://api.mainnet.minepi.com",
                Network::PiTestnet => "https://api.testnet.minepi.com",
                Network::StellarTestnet => "https://horizon-testnet.stellar.org",
            }
        }

        pub fn network_passphrase(&self) -> &'static str {
            match self {
                Network::PiMainnet => "Pi Network",
                Network::PiTestnet => "Pi Testnet",
                Network::StellarTestnet => "Test SDF Network ; September 2015",
            }
        }
    }
    ```

  - **Implement Stellar client in `src/stellar/client.rs`**:

    ````rust
    use crate::{
        client::PiNetworkClient,
        models::{Network, SendAssetsParams, TransactionResponse},
        errors::PiError,
        Result,
    };
    use serde_json::Value;
    use std::collections::HashMap;

    impl PiNetworkClient {
        /// Get account balance for a specific network and account
        ///
        /// # Arguments
        /// * `network` - The network to query (Pi mainnet, testnet, or Stellar testnet)
        /// * `account` - Account ID or secret seed (if starts with 'S')
        ///
        /// # Returns
        /// * `Result<f64>` - Account balance in native currency
        ///
        /// # Example
        /// ```rust
        /// use pi_rust::{PiNetworkClient, Network};
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
        ///     let client = PiNetworkClient::new("your-api-key".to_string())?;
        ///     let balance = client.get_account_balance(
        ///         Network::PiTestnet,
        ///         "GTEST123..."
        ///     ).await?;
        ///     println!("Balance: {} Pi", balance);
        ///     Ok(())
        /// }
        /// ```
        pub async fn get_account_balance(&self, network: Network, account: &str) -> Result<f64> {
            if account.is_empty() {
                return Err(PiError::Configuration("Account cannot be empty".to_string()));
            }

            let account_id = if account.starts_with('S') {
                // Convert secret seed to account ID
                self.secret_to_account_id(account)?
            } else {
                account.to_string()
            };

            let server_url = network.server_url();
            let url = format!("{}/accounts/{}", server_url, account_id);

            let response = self.http_client
                .get(&url)
                .header("Accept", "application/json")
                .send()
                .await
                .map_err(PiError::Http)?;

            if !response.status().is_success() {
                return Err(PiError::Stellar(format!(
                    "Failed to fetch account: HTTP {}",
                    response.status()
                )));
            }

            let account_data: Value = response.json().await.map_err(PiError::Http)?;

            let balances = account_data["balances"]
                .as_array()
                .ok_or_else(|| PiError::Stellar("No balances found in account data".to_string()))?;

            for balance in balances {
                if balance["asset_type"].as_str() == Some("native") {
                    let balance_str = balance["balance"]
                        .as_str()
                        .ok_or_else(|| PiError::Stellar("Invalid balance format".to_string()))?;

                    return balance_str
                        .parse::<f64>()
                        .map_err(|_| PiError::Stellar("Failed to parse balance".to_string()));
                }
            }

            Ok(0.0)
        }

        /// Send native assets (Pi) to another account
        ///
        /// # Arguments
        /// * `params` - Transaction parameters including network, accounts, amount, etc.
        ///
        /// # Returns
        /// * `Result<TransactionResponse>` - Transaction result with hash and details
        ///
        /// # Example
        /// ```rust
        /// use pi_rust::{PiNetworkClient, Network, SendAssetsParams};
        ///
        /// #[tokio::main]
        /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
        ///     let client = PiNetworkClient::new("your-api-key".to_string())?;
        ///     let params = SendAssetsParams {
        ///         network: Network::PiTestnet,
        ///         source_secret: "STEST123...".to_string(),
        ///         destination: "GTEST456...".to_string(),
        ///         amount: 10.0,
        ///         memo: Some("Payment".to_string()),
        ///         fee: None,
        ///     };
        ///
        ///     let result = client.send_native_assets(params).await?;
        ///     println!("Transaction hash: {}", result.hash);
        ///     Ok(())
        /// }
        /// ```
        pub async fn send_native_assets(&self, params: SendAssetsParams) -> Result<TransactionResponse> {
            // Validate parameters
            if params.source_secret.is_empty() {
                return Err(PiError::Configuration("Source secret cannot be empty".to_string()));
            }

            if params.destination.is_empty() {
                return Err(PiError::Configuration("Destination cannot be empty".to_string()));
            }

            if params.amount <= 0.0 {
                return Err(PiError::Configuration("Amount must be positive".to_string()));
            }

            // Get source account ID from secret
            let source_account_id = self.secret_to_account_id(&params.source_secret)?;

            // Check source account balance
            let balance = self.get_account_balance(params.network.clone(), &source_account_id).await?;
            let required_balance = params.amount + 0.01; // Add fee buffer

            if balance < required_balance {
                return Err(PiError::InsufficientBalance {
                    available: balance,
                    required: required_balance,
                });
            }

            // Build and submit transaction
            let transaction = self.build_payment_transaction(&params).await?;
            self.submit_transaction(&params.network, &transaction).await
        }

        /// Convert secret seed to account ID (placeholder - would use actual Stellar SDK)
        fn secret_to_account_id(&self, secret: &str) -> Result<String> {
            // This is a placeholder implementation
            // In a real implementation, you would use the Stellar SDK to convert
            if !secret.starts_with('S') {
                return Err(PiError::Stellar("Invalid secret seed format".to_string()));
            }

            // For now, return a mock account ID
            // In real implementation: stellar_sdk::KeyPair::from_secret_seed(secret)?.account_id()
            Ok(format!("G{}", &secret[1..]))
        }

        /// Build a payment transaction (placeholder implementation)
        async fn build_payment_transaction(&self, params: &SendAssetsParams) -> Result<String> {
            let server_url = params.network.server_url();
            let source_account_id = self.secret_to_account_id(&params.source_secret)?;

            // Get account sequence number
            let account_url = format!("{}/accounts/{}", server_url, source_account_id);
            let account_response = self.http_client
                .get(&account_url)
                .send()
                .await
                .map_err(PiError::Http)?;

            let account_data: Value = account_response.json().await.map_err(PiError::Http)?;
            let sequence = account_data["sequence"]
                .as_str()
                .ok_or_else(|| PiError::Stellar("Failed to get account sequence".to_string()))?;

            // Build transaction envelope (simplified)
            let amount_stroops = (params.amount * 10_000_000.0).floor() as u64;
            let fee = params.fee.unwrap_or(100_000);

            let transaction_data = serde_json::json!({
                "source_account": source_account_id,
                "sequence": sequence,
                "fee": fee,
                "operations": [{
                    "type": "payment",
                    "destination": params.destination,
                    "asset": {"type": "native"},
                    "amount": amount_stroops.to_string()
                }],
                "memo": params.memo.as_ref().unwrap_or(&"".to_string()),
                "network_passphrase": params.network.network_passphrase()
            });

            // In real implementation, this would create and sign a proper Stellar transaction
            Ok(transaction_data.to_string())
        }

        /// Submit transaction to the network
        async fn submit_transaction(&self, network: &Network, transaction: &str) -> Result<TransactionResponse> {
            let server_url = network.server_url();
            let submit_url = format!("{}/transactions", server_url);

            // In real implementation, this would submit the actual signed transaction
            let mut form_data = HashMap::new();
            form_data.insert("tx", transaction);

            let response = self.http_client
                .post(&submit_url)
                .form(&form_data)
                .send()
                .await
                .map_err(PiError::Http)?;

            if !response.status().is_success() {
                let error_text = response.text().await.unwrap_or_default();
                return Err(PiError::Stellar(format!(
                    "Transaction submission failed: {}",
                    error_text
                )));
            }

            let result: Value = response.json().await.map_err(PiError::Http)?;

            Ok(TransactionResponse {
                hash: result["hash"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string(),
                ledger: result["ledger"]
                    .as_u64()
                    .unwrap_or(0) as u32,
                envelope_xdr: result["envelope_xdr"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                result_xdr: result["result_xdr"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                result_meta_xdr: result["result_meta_xdr"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
            })
        }
    }
    ````

  - **Add comprehensive tests in `src/stellar/client.rs`**:

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::config::ClientConfig;
        use wiremock::{MockServer, Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use serde_json::json;

        #[tokio::test]
        async fn test_get_account_balance_success() {
            let mock_server = MockServer::start().await;
            let account_data = json!({
                "account_id": "GTEST123",
                "sequence": "123456789",
                "balances": [
                    {
                        "asset_type": "native",
                        "balance": "100.5000000"
                    }
                ]
            });

            Mock::given(method("GET"))
                .and(path("/accounts/GTEST123"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&account_data))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            // Mock the network to use our test server
            let network = Network::PiTestnet;
            let balance = client.get_account_balance(network, "GTEST123").await.unwrap();
            assert_eq!(balance, 100.5);
        }

        #[tokio::test]
        async fn test_send_native_assets_insufficient_balance() {
            let mock_server = MockServer::start().await;
            let account_data = json!({
                "account_id": "GTEST123",
                "sequence": "123456789",
                "balances": [
                    {
                        "asset_type": "native",
                        "balance": "5.0000000"
                    }
                ]
            });

            Mock::given(method("GET"))
                .and(path("/accounts/GTEST123"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&account_data))
                .mount(&mock_server)
                .await;

            let config = ClientConfig::builder("test-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            let params = SendAssetsParams {
                network: Network::PiTestnet,
                source_secret: "STEST123".to_string(),
                destination: "GTEST456".to_string(),
                amount: 10.0,
                memo: None,
                fee: None,
            };

            let result = client.send_native_assets(params).await;
            match result {
                Err(PiError::InsufficientBalance { available, required }) => {
                    assert_eq!(available, 5.0);
                    assert!(required > 10.0);
                }
                _ => panic!("Expected insufficient balance error"),
            }
        }

        #[tokio::test]
        async fn test_network_configurations() {
            assert_eq!(Network::PiMainnet.server_url(), "https://api.mainnet.minepi.com");
            assert_eq!(Network::PiTestnet.server_url(), "https://api.testnet.minepi.com");
            assert_eq!(Network::StellarTestnet.server_url(), "https://horizon-testnet.stellar.org");

            assert_eq!(Network::PiMainnet.network_passphrase(), "Pi Network");
            assert_eq!(Network::PiTestnet.network_passphrase(), "Pi Testnet");
            assert_eq!(Network::StellarTestnet.network_passphrase(), "Test SDF Network ; September 2015");
        }

        #[tokio::test]
        async fn test_invalid_parameters() {
            let client = PiNetworkClient::new("test-key".to_string()).unwrap();

            // Test empty account
            let result = client.get_account_balance(Network::PiTestnet, "").await;
            assert!(matches!(result, Err(PiError::Configuration(_))));

            // Test invalid amount
            let params = SendAssetsParams {
                network: Network::PiTestnet,
                source_secret: "STEST123".to_string(),
                destination: "GTEST456".to_string(),
                amount: -5.0,
                memo: None,
                fee: None,
            };

            let result = client.send_native_assets(params).await;
            assert!(matches!(result, Err(PiError::Configuration(_))));
        }
    }
    ```

  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 8. Create comprehensive test suite

  - **Create integration test utilities in `tests/common/mod.rs`**:

    ```rust
    use pi_rust::{PiNetworkClient, ClientConfig};
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path, header};
    use serde_json::json;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn init_test_logging() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    pub struct TestContext {
        pub mock_server: MockServer,
        pub client: PiNetworkClient,
    }

    impl TestContext {
        pub async fn new() -> Self {
            init_test_logging();

            let mock_server = MockServer::start().await;
            let config = ClientConfig::builder("test-api-key".to_string())
                .base_url(mock_server.uri().parse().unwrap())
                .build();
            let client = PiNetworkClient::with_config(config).unwrap();

            Self {
                mock_server,
                client,
            }
        }

        pub async fn setup_user_profile_mock(&self, access_token: &str, user_uid: &str) {
            let profile = json!({
                "accessToken": access_token,
                "user": {
                    "uid": user_uid,
                    "username": "testuser",
                    "credentials": {
                        "scopes": ["payments"],
                        "valid_until": {
                            "timestamp": 9999999999,
                            "iso8601": "2099-01-01T00:00:00Z"
                        }
                    }
                }
            });

            Mock::given(method("GET"))
                .and(path("/me"))
                .and(header("authorization", format!("Bearer {}", access_token)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&profile))
                .mount(&self.mock_server)
                .await;
        }

        pub async fn setup_payment_creation_mock(&self, payment_id: &str, user_uid: &str, amount: f64) {
            let payment = json!({
                "identifier": payment_id,
                "user_uid": user_uid,
                "amount": amount,
                "memo": "Test payment",
                "metadata": null,
                "from_address": null,
                "to_address": "GTEST456",
                "created_at": "2023-01-01T00:00:00Z",
                "direction": "user_to_app",
                "network": "Pi Testnet",
                "status": {
                    "developer_approved": false,
                    "transaction_verified": false,
                    "developer_completed": false,
                    "cancelled": false,
                    "user_cancelled": false
                },
                "transaction": null
            });

            Mock::given(method("POST"))
                .and(path("/payments"))
                .and(header("authorization", "Key test-api-key"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&payment))
                .mount(&self.mock_server)
                .await;
        }

        pub async fn setup_payment_status_mocks(&self, payment_id: &str) {
            let base_payment = json!({
                "identifier": payment_id,
                "user_uid": "user123",
                "amount": 10.0,
                "memo": "Test payment",
                "metadata": null,
                "from_address": "GTEST123",
                "to_address": "GTEST456",
                "created_at": "2023-01-01T00:00:00Z",
                "direction": "user_to_app",
                "network": "Pi Testnet",
                "transaction": null
            });

            // Get payment mock
            Mock::given(method("GET"))
                .and(path(format!("/payments/{}", payment_id)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&json!({
                    **base_payment.as_object().unwrap().clone(),
                    "status": {
                        "developer_approved": false,
                        "transaction_verified": false,
                        "developer_completed": false,
                        "cancelled": false,
                        "user_cancelled": false
                    }
                })))
                .mount(&self.mock_server)
                .await;

            // Approve payment mock
            Mock::given(method("POST"))
                .and(path(format!("/payments/{}/approve", payment_id)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&json!({
                    **base_payment.as_object().unwrap().clone(),
                    "status": {
                        "developer_approved": true,
                        "transaction_verified": false,
                        "developer_completed": false,
                        "cancelled": false,
                        "user_cancelled": false
                    }
                })))
                .mount(&self.mock_server)
                .await;

            // Complete payment mock
            Mock::given(method("POST"))
                .and(path(format!("/payments/{}/complete", payment_id)))
                .respond_with(ResponseTemplate::new(200).set_body_json(&json!({
                    **base_payment.as_object().unwrap().clone(),
                    "status": {
                        "developer_approved": true,
                        "transaction_verified": true,
                        "developer_completed": true,
                        "cancelled": false,
                        "user_cancelled": false
                    },
                    "transaction": {
                        "txid": "tx123",
                        "verified": true,
                        "_link": "https://stellar.expert/explorer/testnet/tx/tx123"
                    }
                })))
                .mount(&self.mock_server)
                .await;
        }
    }
    ```

  - **Create end-to-end workflow test in `tests/integration_tests.rs`**:

    ```rust
    mod common;

    use common::TestContext;
    use pi_rust::{PiError};

    #[tokio::test]
    async fn test_complete_payment_workflow() {
        let ctx = TestContext::new().await;
        let payment_id = "payment123";
        let user_uid = "user123";
        let access_token = "valid-token";

        // Setup all required mocks
        ctx.setup_user_profile_mock(access_token, user_uid).await;
        ctx.setup_payment_creation_mock(payment_id, user_uid, 10.0).await;
        ctx.setup_payment_status_mocks(payment_id).await;

        // Step 1: Authenticate user
        let profile = ctx.client.get_user_profile(access_token).await.unwrap();
        assert_eq!(profile.user.uid, user_uid);

        // Step 2: Create payment
        let payment = ctx.client.create_payment(
            10.0,
            Some("Test payment".to_string()),
            None,
            user_uid.to_string(),
        ).await.unwrap();
        assert_eq!(payment.identifier, payment_id);
        assert_eq!(payment.amount, 10.0);
        assert!(!payment.status.developer_approved);

        // Step 3: Get payment details
        let retrieved_payment = ctx.client.get_payment(payment_id).await.unwrap();
        assert_eq!(retrieved_payment.identifier, payment_id);

        // Step 4: Approve payment
        let approved_payment = ctx.client.approve_payment(payment_id).await.unwrap();
        assert!(approved_payment.status.developer_approved);

        // Step 5: Complete payment with transaction
        let completed_payment = ctx.client.complete_payment(payment_id, "tx123").await.unwrap();
        assert!(completed_payment.status.developer_completed);
        assert!(completed_payment.status.transaction_verified);
        assert_eq!(
            completed_payment.transaction.as_ref().unwrap().tx_id.as_ref().unwrap(),
            "tx123"
        );
    }

    #[tokio::test]
    async fn test_error_scenarios() {
        let ctx = TestContext::new().await;

        // Test invalid API key
        let result = ctx.client.get_payment("nonexistent").await;
        assert!(result.is_err());

        // Test invalid access token
        let result = ctx.client.get_user_profile("invalid-token").await;
        assert!(matches!(result, Err(PiError::Authentication(_))));

        // Test invalid payment parameters
        let result = ctx.client.create_payment(-5.0, None, None, "user123".to_string()).await;
        assert!(matches!(result, Err(PiError::Configuration(_))));

        // Test empty identifiers
        let result = ctx.client.get_payment("").await;
        assert!(matches!(result, Err(PiError::Configuration(_))));
    }

    #[tokio::test]
    async fn test_network_resilience() {
        let ctx = TestContext::new().await;

        // Test timeout handling
        use wiremock::{Mock, ResponseTemplate};
        use wiremock::matchers::{method, path};
        use std::time::Duration;

        Mock::given(method("GET"))
            .and(path("/slow"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_delay(Duration::from_secs(2))
                    .set_body_json(&serde_json::json!({"slow": "response"}))
            )
            .mount(&ctx.mock_server)
            .await;

        // This should succeed with default timeout
        let request = ctx.client.get("/slow");
        let result: Result<serde_json::Value, _> = ctx.client.execute_request(request).await;
        // Depending on timeout configuration, this might succeed or fail
        println!("Slow request result: {:?}", result);
    }
    ```

  - **Create performance benchmarks in `benches/performance.rs`**:

    ```rust
    use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
    use pi_rust::{PiNetworkClient, ClientConfig};
    use tokio::runtime::Runtime;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use serde_json::json;

    async fn setup_mock_server() -> (MockServer, PiNetworkClient) {
        let mock_server = MockServer::start().await;

        // Setup payment creation mock
        let payment = json!({
            "identifier": "bench-payment",
            "user_uid": "bench-user",
            "amount": 10.0,
            "memo": "Benchmark payment",
            "status": {
                "developer_approved": false,
                "transaction_verified": false,
                "developer_completed": false,
                "cancelled": false,
                "user_cancelled": false
            }
        });

        Mock::given(method("POST"))
            .and(path("/payments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&payment))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/payments/bench-payment"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&payment))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::builder("bench-key".to_string())
            .base_url(mock_server.uri().parse().unwrap())
            .build();
        let client = PiNetworkClient::with_config(config).unwrap();

        (mock_server, client)
    }

    fn benchmark_payment_creation(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        let (_mock_server, client) = rt.block_on(setup_mock_server());

        c.bench_function("create_payment", |b| {
            b.to_async(&rt).iter(|| async {
                let result = client.create_payment(
                    black_box(10.0),
                    black_box(Some("Benchmark".to_string())),
                    black_box(None),
                    black_box("bench-user".to_string()),
                ).await;
                black_box(result)
            })
        });
    }

    fn benchmark_payment_retrieval(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        let (_mock_server, client) = rt.block_on(setup_mock_server());

        c.bench_function("get_payment", |b| {
            b.to_async(&rt).iter(|| async {
                let result = client.get_payment(black_box("bench-payment")).await;
                black_box(result)
            })
        });
    }

    fn benchmark_concurrent_requests(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        let (_mock_server, client) = rt.block_on(setup_mock_server());

        let mut group = c.benchmark_group("concurrent_requests");

        for concurrency in [1, 5, 10, 20].iter() {
            group.bench_with_input(
                BenchmarkId::new("get_payment", concurrency),
                concurrency,
                |b, &concurrency| {
                    b.to_async(&rt).iter(|| async {
                        let futures: Vec<_> = (0..concurrency)
                            .map(|_| client.get_payment("bench-payment"))
                            .collect();

                        let results = futures::future::join_all(futures).await;
                        black_box(results)
                    })
                },
            );
        }
        group.finish();
    }

    criterion_group!(
        benches,
        benchmark_payment_creation,
        benchmark_payment_retrieval,
        benchmark_concurrent_requests
    );
    criterion_main!(benches);
    ```

  - **Add benchmark configuration to `Cargo.toml`**:

    ```toml
    [[bench]]
    name = "performance"
    harness = false

    [dev-dependencies]
    criterion = { version = "0.5", features = ["html_reports"] }
    futures = "0.3"
    env_logger = "0.10"
    ```

  - **Create test runner script `scripts/run_tests.sh`**:

    ```bash
    #!/bin/bash
    set -e

    echo "Running unit tests..."
    cargo test --lib

    echo "Running integration tests..."
    cargo test --test integration_tests

    echo "Running benchmarks..."
    cargo bench

    echo "Generating test coverage..."
    cargo tarpaulin --out Html --output-dir coverage/

    echo "All tests completed successfully!"
    ```

  - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

- [ ] 9. Add documentation and examples

  - **Create comprehensive README.md**:

    ````markdown
    # Pi Network Rust SDK

    A comprehensive Rust SDK for integrating with Pi Network APIs, enabling server-side applications to manage payments, authenticate users, and interact with the Stellar blockchain.

    ## Features

    - 🔐 **User Authentication** - Validate access tokens and retrieve user profiles
    - 💰 **Payment Management** - Create, approve, cancel, and complete payments
    - 🌟 **Stellar Integration** - Send native assets and query account balances
    - 🔄 **Async/Await Support** - Built on tokio for high-performance async operations
    - 🛡️ **Type Safety** - Strongly-typed APIs with comprehensive error handling
    - 🧪 **Well Tested** - Extensive unit and integration test coverage

    ## Quick Start

    Add this to your `Cargo.toml`:

    ```toml
    [dependencies]
    pi-rust = "0.1.0"
    tokio = { version = "1.0", features = ["full"] }
    ```
    ````

    ### Basic Usage

    ```rust
    use pi_rust::{PiNetworkClient, Network};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize the client with your API key
        let client = PiNetworkClient::new("your-api-key".to_string())?;

        // Authenticate a user
        let profile = client.get_user_profile("user-access-token").await?;
        println!("User: {} ({})", profile.user.username, profile.user.uid);

        // Create a payment
        let payment = client.create_payment(
            10.5,                                    // amount
            Some("Coffee purchase".to_string()),     // memo
            None,                                    // metadata
            profile.user.uid,                        // user ID
        ).await?;

        println!("Payment created: {}", payment.identifier);

        // Approve the payment
        let approved = client.approve_payment(&payment.identifier).await?;
        println!("Payment approved: {}", approved.status.developer_approved);

        Ok(())
    }
    ```

    ## Examples

    See the [examples](examples/) directory for more comprehensive usage examples:

    - [Basic Payment Flow](examples/basic_payment.rs)
    - [Stellar Transactions](examples/stellar_operations.rs)
    - [Error Handling](examples/error_handling.rs)
    - [Configuration Options](examples/configuration.rs)

    ## Documentation

    - [API Documentation](https://docs.rs/pi-rust)
    - [Getting Started Guide](docs/getting-started.md)
    - [Payment Integration Guide](docs/payment-integration.md)
    - [Stellar Operations Guide](docs/stellar-operations.md)
    - [Error Handling Guide](docs/error-handling.md)

    ## Contributing

    We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

    ## License

    This project is licensed under the MIT OR Apache-2.0 license.

    ```

    ```

  - **Create example files in `examples/` directory**:

    **`examples/basic_payment.rs`**:

    ```rust
    //! Basic payment workflow example
    //!
    //! This example demonstrates the complete payment lifecycle:
    //! 1. User authentication
    //! 2. Payment creation
    //! 3. Payment approval
    //! 4. Payment completion

    use pi_rust::{PiNetworkClient, PiError};
    use std::env;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Get API key from environment
        let api_key = env::var("PI_API_KEY")
            .expect("PI_API_KEY environment variable must be set");

        // Initialize client
        let client = PiNetworkClient::new(api_key)?;

        // Example access token (in real app, this comes from Pi Network auth)
        let access_token = env::var("PI_ACCESS_TOKEN")
            .expect("PI_ACCESS_TOKEN environment variable must be set");

        println!("🔐 Authenticating user...");

        // Step 1: Authenticate user and get profile
        let profile = match client.get_user_profile(&access_token).await {
            Ok(profile) => {
                println!("✅ User authenticated: {} ({})", profile.user.username, profile.user.uid);
                profile
            }
            Err(PiError::Authentication(msg)) => {
                eprintln!("❌ Authentication failed: {}", msg);
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        };

        println!("\n💰 Creating payment...");

        // Step 2: Create a payment
        let payment = client.create_payment(
            15.75,                                      // amount in Pi
            Some("Premium subscription".to_string()),   // memo
            Some(serde_json::json!({                   // metadata
                "product_id": "premium_monthly",
                "customer_tier": "gold"
            })),
            profile.user.uid,                          // user ID
        ).await?;

        println!("✅ Payment created:");
        println!("   ID: {}", payment.identifier);
        println!("   Amount: {} Pi", payment.amount);
        println!("   Status: Developer Approved = {}", payment.status.developer_approved);

        println!("\n📋 Retrieving payment details...");

        // Step 3: Get payment details
        let retrieved = client.get_payment(&payment.identifier).await?;
        println!("✅ Payment retrieved: {} Pi to {}",
                retrieved.amount,
                retrieved.to_address.as_deref().unwrap_or("pending"));

        println!("\n✅ Approving payment...");

        // Step 4: Approve the payment
        let approved = client.approve_payment(&payment.identifier).await?;
        println!("✅ Payment approved: {}", approved.status.developer_approved);

        // In a real application, you would wait for the user to complete
        // the payment in the Pi app, then complete it with the transaction ID

        println!("\n🎉 Payment workflow completed successfully!");
        println!("   Next: Wait for user to complete payment in Pi app");
        println!("   Then: Call complete_payment() with the transaction ID");

        Ok(())
    }
    ```

    **`examples/stellar_operations.rs`**:

    ```rust
    //! Stellar blockchain operations example
    //!
    //! This example shows how to:
    //! 1. Check account balances
    //! 2. Send native Pi assets
    //! 3. Handle different networks

    use pi_rust::{PiNetworkClient, Network, SendAssetsParams, PiError};
    use std::env;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("PI_API_KEY")
            .expect("PI_API_KEY environment variable must be set");

        let client = PiNetworkClient::new(api_key)?;

        // Example accounts (use your own testnet accounts)
        let source_secret = env::var("SOURCE_SECRET")
            .expect("SOURCE_SECRET environment variable must be set");
        let destination_account = env::var("DESTINATION_ACCOUNT")
            .expect("DESTINATION_ACCOUNT environment variable must be set");

        println!("🌟 Stellar Operations Example");
        println!("==============================\n");

        // Step 1: Check account balance
        println!("💰 Checking account balance...");

        match client.get_account_balance(Network::PiTestnet, &destination_account).await {
            Ok(balance) => {
                println!("✅ Account balance: {} Pi", balance);
            }
            Err(PiError::Stellar(msg)) => {
                println!("⚠️  Account not found or network error: {}", msg);
                println!("   Make sure the account exists and is funded");
                return Ok(());
            }
            Err(e) => return Err(e.into()),
        }

        println!("\n🚀 Sending native assets...");

        // Step 2: Send Pi to another account
        let send_params = SendAssetsParams {
            network: Network::PiTestnet,
            source_secret: source_secret.clone(),
            destination: destination_account.clone(),
            amount: 5.0,
            memo: Some("Rust SDK test transaction".to_string()),
            fee: Some(100_000), // 0.01 Pi fee
        };

        match client.send_native_assets(send_params).await {
            Ok(transaction) => {
                println!("✅ Transaction successful!");
                println!("   Hash: {}", transaction.hash);
                println!("   Ledger: {}", transaction.ledger);
            }
            Err(PiError::InsufficientBalance { available, required }) => {
                println!("❌ Insufficient balance:");
                println!("   Available: {} Pi", available);
                println!("   Required: {} Pi", required);
            }
            Err(PiError::Stellar(msg)) => {
                println!("❌ Stellar operation failed: {}", msg);
            }
            Err(e) => return Err(e.into()),
        }

        println!("\n🌐 Network Information:");
        println!("   Pi Mainnet: {}", Network::PiMainnet.server_url());
        println!("   Pi Testnet: {}", Network::PiTestnet.server_url());
        println!("   Stellar Testnet: {}", Network::StellarTestnet.server_url());

        Ok(())
    }
    ```

    **`examples/error_handling.rs`**:

    ```rust
    //! Error handling patterns example
    //!
    //! This example demonstrates how to handle different types of errors
    //! that can occur when using the Pi Network SDK

    use pi_rust::{PiNetworkClient, PiError, Network, SendAssetsParams};
    use std::env;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let api_key = env::var("PI_API_KEY").unwrap_or_else(|_| "invalid-key".to_string());
        let client = PiNetworkClient::new(api_key)?;

        println!("🚨 Error Handling Examples");
        println!("==========================\n");

        // Example 1: Authentication errors
        println!("1️⃣ Testing authentication errors...");

        match client.get_user_profile("invalid-token").await {
            Ok(_) => println!("   Unexpected success"),
            Err(PiError::Authentication(msg)) => {
                println!("   ✅ Caught authentication error: {}", msg);
            }
            Err(e) => println!("   ❓ Other error: {}", e),
        }

        // Example 2: Configuration errors
        println!("\n2️⃣ Testing configuration errors...");

        match client.create_payment(-10.0, None, None, "user123".to_string()).await {
            Ok(_) => println!("   Unexpected success"),
            Err(PiError::Configuration(msg)) => {
                println!("   ✅ Caught configuration error: {}", msg);
            }
            Err(e) => println!("   ❓ Other error: {}", e),
        }

        // Example 3: Pi Network API errors
        println!("\n3️⃣ Testing Pi Network API errors...");

        match client.get_payment("nonexistent-payment").await {
            Ok(_) => println!("   Unexpected success"),
            Err(PiError::PiNetwork { error_name, error_message, .. }) => {
                println!("   ✅ Caught Pi Network error:");
                println!("      Error: {}", error_name);
                println!("      Message: {}", error_message);
            }
            Err(e) => println!("   ❓ Other error: {}", e),
        }

        // Example 4: Stellar operation errors
        println!("\n4️⃣ Testing Stellar operation errors...");

        let params = SendAssetsParams {
            network: Network::PiTestnet,
            source_secret: "SINVALID123".to_string(),
            destination: "GINVALID456".to_string(),
            amount: 1000000.0, // Very large amount
            memo: None,
            fee: None,
        };

        match client.send_native_assets(params).await {
            Ok(_) => println!("   Unexpected success"),
            Err(PiError::InsufficientBalance { available, required }) => {
                println!("   ✅ Caught insufficient balance error:");
                println!("      Available: {} Pi", available);
                println!("      Required: {} Pi", required);
            }
            Err(PiError::Stellar(msg)) => {
                println!("   ✅ Caught Stellar error: {}", msg);
            }
            Err(e) => println!("   ❓ Other error: {}", e),
        }

        // Example 5: HTTP/Network errors
        println!("\n5️⃣ Testing network errors...");

        // Create client with invalid base URL to simulate network error
        use pi_rust::ClientConfig;
        use url::Url;

        let bad_config = ClientConfig::builder("test-key".to_string())
            .base_url(Url::parse("https://nonexistent-domain-12345.com").unwrap())
            .build();

        let bad_client = PiNetworkClient::with_config(bad_config)?;

        match bad_client.get_payment("test").await {
            Ok(_) => println!("   Unexpected success"),
            Err(PiError::Http(e)) => {
                println!("   ✅ Caught HTTP error: {}", e);
            }
            Err(e) => println!("   ❓ Other error: {}", e),
        }

        println!("\n✨ Error handling examples completed!");
        println!("\n💡 Best Practices:");
        println!("   • Always handle specific error types when possible");
        println!("   • Use pattern matching to handle different error scenarios");
        println!("   • Log errors appropriately for debugging");
        println!("   • Provide meaningful error messages to users");
        println!("   • Consider retry logic for transient network errors");

        Ok(())
    }
    ```

  - **Create documentation files in `docs/` directory**:

    **`docs/getting-started.md`**:

    ````markdown
    # Getting Started with Pi Network Rust SDK

    This guide will help you get up and running with the Pi Network Rust SDK quickly.

    ## Prerequisites

    - Rust 1.70 or later
    - A Pi Network API key (obtain from Pi Developer Portal)
    - Basic familiarity with async/await in Rust

    ## Installation

    Add the SDK to your `Cargo.toml`:

    ```toml
    [dependencies]
    pi-rust = "0.1.0"
    tokio = { version = "1.0", features = ["full"] }
    serde_json = "1.0"  # For metadata handling
    ```
    ````

    ## Basic Setup

    ```rust
    use pi_rust::PiNetworkClient;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize the client
        let client = PiNetworkClient::new("your-api-key".to_string())?;

        // Your code here...

        Ok(())
    }
    ```

    ## Configuration Options

    For more control over the client behavior:

    ```rust
    use pi_rust::{PiNetworkClient, ClientConfig};
    use std::time::Duration;
    use url::Url;

    let config = ClientConfig::builder("your-api-key".to_string())
        .base_url(Url::parse("https://api.minepi.com/v2").unwrap())
        .timeout(Duration::from_secs(30))
        .build();

    let client = PiNetworkClient::with_config(config)?;
    ```

    ## Next Steps

    - [Payment Integration Guide](payment-integration.md)
    - [Stellar Operations Guide](stellar-operations.md)
    - [Error Handling Guide](error-handling.md)
    - [API Reference](https://docs.rs/pi-rust)

    ```

    ```

  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5_

- [ ] 10. Set up CI/CD pipeline and quality checks

  - **Create comprehensive CI workflow in `.github/workflows/ci.yml`**:

    ```yaml
    name: CI

    on:
      push:
        branches: [main, develop]
      pull_request:
        branches: [main, develop]

    env:
      CARGO_TERM_COLOR: always
      RUST_BACKTRACE: 1

    jobs:
      check:
        name: Check
        runs-on: ubuntu-latest
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable

          - name: Run cargo check
            run: cargo check --all-targets --all-features

      test:
        name: Test Suite
        runs-on: ubuntu-latest
        strategy:
          matrix:
            rust: [stable, beta, 1.70.0] # MSRV
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install ${{ matrix.rust }} toolchain
            uses: dtolnay/rust-toolchain@master
            with:
              toolchain: ${{ matrix.rust }}

          - name: Run cargo test
            run: cargo test --all-features --workspace

          - name: Run integration tests
            run: cargo test --test integration_tests

      lints:
        name: Lints
        runs-on: ubuntu-latest
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable
            with:
              components: rustfmt, clippy

          - name: Run cargo fmt
            run: cargo fmt --all -- --check

          - name: Run cargo clippy
            run: cargo clippy --all-targets --all-features -- -D warnings

      docs:
        name: Documentation
        runs-on: ubuntu-latest
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable

          - name: Check documentation
            run: cargo doc --no-deps --document-private-items --all-features
            env:
              RUSTDOCFLAGS: "-D warnings"

      security:
        name: Security Audit
        runs-on: ubuntu-latest
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install cargo-audit
            run: cargo install cargo-audit

          - name: Run security audit
            run: cargo audit

      coverage:
        name: Code Coverage
        runs-on: ubuntu-latest
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable

          - name: Install cargo-tarpaulin
            run: cargo install cargo-tarpaulin

          - name: Generate code coverage
            run: cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out xml

          - name: Upload to codecov.io
            uses: codecov/codecov-action@v3
            with:
              file: cobertura.xml
              fail_ci_if_error: true

      benchmark:
        name: Benchmarks
        runs-on: ubuntu-latest
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable

          - name: Run benchmarks
            run: cargo bench --all-features

          - name: Store benchmark result
            uses: benchmark-action/github-action-benchmark@v1
            with:
              tool: "cargo"
              output-file-path: target/criterion/reports/index.html
              github-token: ${{ secrets.GITHUB_TOKEN }}
              auto-push: true
    ```

  - **Create release workflow in `.github/workflows/release.yml`**:

    ```yaml
    name: Release

    on:
      push:
        tags:
          - "v*"

    jobs:
      create-release:
        name: Create Release
        runs-on: ubuntu-latest
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable

          - name: Run tests
            run: cargo test --all-features --workspace

          - name: Build release
            run: cargo build --release --all-features

          - name: Create Release
            uses: softprops/action-gh-release@v1
            with:
              body: |
                Changes in this Release
                - First Change
                - Second Change
              draft: false
              prerelease: false
            env:
              GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}

      publish-crate:
        name: Publish to crates.io
        runs-on: ubuntu-latest
        needs: create-release
        steps:
          - name: Checkout sources
            uses: actions/checkout@v4

          - name: Install stable toolchain
            uses: dtolnay/rust-toolchain@stable

          - name: Publish to crates.io
            run: cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}
    ```

  - **Create dependabot configuration in `.github/dependabot.yml`**:
    ```yaml
    version: 2
    updates:
      - package-ecosystem: "cargo"
        directory: "/"
        schedule:
          interval: "weekly"
        open-pull-requests-limit: 10
        reviewers:
          - "maintainer-username"
        assignees:
          - "maintainer-username"
        commit-message:
          prefix: "deps"
          include: "scope"
    ```
  - **Create issue templates in `.github/ISSUE_TEMPLATE/`**:

    **`.github/ISSUE_TEMPLATE/bug_report.yml`**:

    ```yaml
    name: Bug Report
    description: File a bug report
    title: "[Bug]: "
    labels: ["bug", "triage"]
    body:
      - type: markdown
        attributes:
          value: |
            Thanks for taking the time to fill out this bug report!
      - type: input
        id: version
        attributes:
          label: Version
          description: What version of pi-rust are you using?
          placeholder: ex. 0.1.0
        validations:
          required: true
      - type: textarea
        id: what-happened
        attributes:
          label: What happened?
          description: Also tell us, what did you expect to happen?
          placeholder: Tell us what you see!
        validations:
          required: true
      - type: textarea
        id: reproduction
        attributes:
          label: Steps to Reproduce
          description: Please provide a minimal code example that reproduces the issue
          placeholder: |
            1. Create client with...
            2. Call method...
            3. See error...
        validations:
          required: true
      - type: textarea
        id: logs
        attributes:
          label: Relevant log output
          description: Please copy and paste any relevant log output. This will be automatically formatted into code, so no need for backticks.
          render: shell
    ```

    **`.github/ISSUE_TEMPLATE/feature_request.yml`**:

    ```yaml
    name: Feature Request
    description: Suggest an idea for this project
    title: "[Feature]: "
    labels: ["enhancement", "triage"]
    body:
      - type: markdown
        attributes:
          value: |
            Thanks for suggesting a new feature!
      - type: textarea
        id: problem
        attributes:
          label: Is your feature request related to a problem?
          description: A clear and concise description of what the problem is.
          placeholder: I'm always frustrated when...
      - type: textarea
        id: solution
        attributes:
          label: Describe the solution you'd like
          description: A clear and concise description of what you want to happen.
      - type: textarea
        id: alternatives
        attributes:
          label: Describe alternatives you've considered
          description: A clear and concise description of any alternative solutions or features you've considered.
      - type: textarea
        id: additional-context
        attributes:
          label: Additional context
          description: Add any other context or screenshots about the feature request here.
    ```

  - **Create pull request template in `.github/pull_request_template.md`**:

    ```markdown
    ## Description

    Brief description of the changes in this PR.

    ## Type of Change

    - [ ] Bug fix (non-breaking change which fixes an issue)
    - [ ] New feature (non-breaking change which adds functionality)
    - [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
    - [ ] Documentation update
    - [ ] Performance improvement
    - [ ] Code refactoring

    ## Testing

    - [ ] I have added tests that prove my fix is effective or that my feature works
    - [ ] New and existing unit tests pass locally with my changes
    - [ ] I have added integration tests for new functionality

    ## Documentation

    - [ ] I have updated the documentation accordingly
    - [ ] I have added rustdoc comments for new public APIs
    - [ ] I have updated examples if necessary

    ## Checklist

    - [ ] My code follows the style guidelines of this project
    - [ ] I have performed a self-review of my own code
    - [ ] I have commented my code, particularly in hard-to-understand areas
    - [ ] My changes generate no new warnings
    - [ ] I have checked my code and corrected any misspellings

    ## Related Issues

    Closes #(issue number)
    ```

  - **Create rustfmt configuration in `rustfmt.toml`**:
    ```toml
    max_width = 100
    hard_tabs = false
    tab_spaces = 4
    newline_style = "Unix"
    use_small_heuristics = "Default"
    reorder_imports = true
    reorder_modules = true
    remove_nested_parens = true
    edition = "2021"
    merge_derives = true
    use_try_shorthand = false
    use_field_init_shorthand = false
    force_explicit_abi = true
    empty_item_single_line = true
    struct_lit_single_line = true
    fn_single_line = false
    where_single_line = false
    imports_layout = "Mixed"
    merge_imports = false
    ```
  - **Create clippy configuration in `clippy.toml`**:

    ```toml
    # Clippy configuration for pi-rust

    # Deny these lints
    disallowed-methods = [
        "std::env::var",  # Use env::var with proper error handling
    ]

    # Maximum cognitive complexity for functions
    cognitive-complexity-threshold = 30

    # Maximum number of lines for functions
    too-many-lines-threshold = 100

    # Maximum number of arguments for functions
    too-many-arguments-threshold = 7

    # Avoid false positives for async functions
    avoid-breaking-exported-api = false
    ```

  - _Requirements: 11.1, 11.2, 11.3, 11.4, 11.5, 11.6, 11.7, 11.8, 11.9_

- [ ] 11. Create open source project infrastructure

  - **Create comprehensive CONTRIBUTING.md**:

    ````markdown
    # Contributing to Pi Network Rust SDK

    We love your input! We want to make contributing to this project as easy and transparent as possible.

    ## Development Process

    We use GitHub to host code, track issues and feature requests, and accept pull requests.

    ## Getting Started

    1. Fork the repo and create your branch from `main`
    2. Install Rust (1.70 or later)
    3. Clone your fork: `git clone https://github.com/yourusername/pi-rust.git`
    4. Install dependencies: `cargo build`
    5. Run tests: `cargo test`

    ## Development Setup

    ```bash
    # Install required tools
    cargo install cargo-tarpaulin  # For coverage
    cargo install cargo-audit     # For security audits
    rustup component add clippy rustfmt

    # Run the full test suite
    ./scripts/run_tests.sh
    ```
    ````

    ## Code Style

    - Use `cargo fmt` to format your code
    - Use `cargo clippy` to catch common mistakes
    - Follow Rust naming conventions
    - Add rustdoc comments for all public APIs
    - Write tests for new functionality

    ## Pull Request Process

    1. Update the README.md with details of changes if needed
    2. Update documentation and examples
    3. Add tests for new functionality
    4. Ensure all tests pass
    5. Make sure your code lints without errors
    6. Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/)

    ## Issue Reporting

    Use GitHub issues to report bugs or request features. Please use the provided templates.

    ## License

    By contributing, you agree that your contributions will be licensed under the same license as the project.

    ```

    ```

  - **Create CODE_OF_CONDUCT.md**:

    ```markdown
    # Contributor Covenant Code of Conduct

    ## Our Pledge

    We as members, contributors, and leaders pledge to make participation in our
    community a harassment-free experience for everyone, regardless of age, body
    size, visible or invisible disability, ethnicity, sex characteristics, gender
    identity and expression, level of experience, education, socio-economic status,
    nationality, personal appearance, race, religion, or sexual identity
    and orientation.

    ## Our Standards

    Examples of behavior that contributes to a positive environment:

    - Using welcoming and inclusive language
    - Being respectful of differing viewpoints and experiences
    - Gracefully accepting constructive criticism
    - Focusing on what is best for the community
    - Showing empathy towards other community members

    ## Enforcement

    Instances of abusive, harassing, or otherwise unacceptable behavior may be
    reported to the community leaders responsible for enforcement at
    [INSERT CONTACT METHOD].

    This Code of Conduct is adapted from the [Contributor Covenant][homepage],
    version 2.0, available at
    https://www.contributor-covenant.org/version/2/0/code_of_conduct.html.
    ```

  - **Create CHANGELOG.md**:

    ```markdown
    # Changelog

    All notable changes to this project will be documented in this file.

    The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
    and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

    ## [Unreleased]

    ### Added

    - Initial release of Pi Network Rust SDK
    - User authentication support
    - Payment management operations
    - Stellar blockchain integration
    - Comprehensive error handling
    - Async/await support throughout
    - Extensive test coverage
    - Documentation and examples

    ## [0.1.0] - 2024-01-01

    ### Added

    - Initial release
    ```

  - **Create LICENSE files** (dual license):

    **`LICENSE-MIT`**:

    ```
    MIT License

    Copyright (c) 2024 Pi Network Rust SDK Contributors

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
    ```

  - **Create repository setup script `scripts/setup_repo.sh`**:

    ```bash
    #!/bin/bash
    # Repository setup script

    set -e

    echo "Setting up Pi Network Rust SDK repository..."

    # Enable branch protection for main branch
    echo "⚠️  Manual step required:"
    echo "   Go to GitHub repository settings and enable branch protection for 'main' branch"
    echo "   Require pull request reviews before merging"
    echo "   Require status checks to pass before merging"
    echo "   Require branches to be up to date before merging"
    echo "   Include administrators in restrictions"

    # Create necessary directories
    mkdir -p .github/workflows
    mkdir -p .github/ISSUE_TEMPLATE
    mkdir -p docs
    mkdir -p examples
    mkdir -p scripts
    mkdir -p tests/common
    mkdir -p benches

    echo "✅ Repository structure created"
    echo "✅ Next steps:"
    echo "   1. Set up GitHub repository secrets:"
    echo "      - CRATES_IO_TOKEN (for publishing)"
    echo "      - CODECOV_TOKEN (for coverage reporting)"
    echo "   2. Enable GitHub Pages for documentation"
    echo "   3. Configure branch protection rules"
    echo "   4. Add repository topics: rust, pi-network, blockchain, stellar, payments"
    ```

  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

- [ ] 12. Implement advanced features and optimizations

  - **Add connection pooling in `src/client.rs`**:

    ```rust
    // Update ClientConfig to include connection pool settings
    #[derive(Debug, Clone)]
    pub struct ClientConfig {
        pub api_key: String,
        pub base_url: Url,
        pub timeout: Duration,
        pub retry_config: RetryConfig,
        pub user_agent: String,
        pub connection_pool_config: ConnectionPoolConfig,
    }

    #[derive(Debug, Clone)]
    pub struct ConnectionPoolConfig {
        pub max_idle_per_host: usize,
        pub max_connections_per_host: usize,
        pub idle_timeout: Duration,
        pub connection_timeout: Duration,
    }

    impl Default for ConnectionPoolConfig {
        fn default() -> Self {
            Self {
                max_idle_per_host: 10,
                max_connections_per_host: 50,
                idle_timeout: Duration::from_secs(90),
                connection_timeout: Duration::from_secs(10),
            }
        }
    }

    // Update client creation to use connection pool
    impl PiNetworkClient {
        pub fn with_config(config: ClientConfig) -> Result<Self> {
            let http_client = Client::builder()
                .timeout(config.timeout)
                .user_agent(&config.user_agent)
                .pool_idle_timeout(config.connection_pool_config.idle_timeout)
                .pool_max_idle_per_host(config.connection_pool_config.max_idle_per_host)
                .connect_timeout(config.connection_pool_config.connection_timeout)
                .build()
                .map_err(PiError::Http)?;

            Ok(Self {
                http_client,
                config,
            })
        }
    }
    ```

  - **Add optional tracing support in `src/lib.rs`**:

    ```rust
    // Add to Cargo.toml features
    // [features]
    // default = []
    // tracing = ["dep:tracing", "dep:tracing-subscriber"]

    #[cfg(feature = "tracing")]
    use tracing::{debug, error, info, warn, instrument};

    // Add tracing to key methods
    impl PiNetworkClient {
        #[cfg_attr(feature = "tracing", instrument(skip(self)))]
        pub async fn create_payment(
            &self,
            amount: f64,
            memo: Option<String>,
            metadata: Option<serde_json::Value>,
            user_uid: String,
        ) -> Result<Payment> {
            #[cfg(feature = "tracing")]
            info!("Creating payment for user {} with amount {}", user_uid, amount);

            // ... existing implementation

            #[cfg(feature = "tracing")]
            debug!("Payment created successfully: {}", payment.identifier);

            Ok(payment)
        }
    }
    ```

  - **Add request caching for read operations in `src/cache.rs`**:

    ```rust
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};
    use std::time::{Duration, Instant};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    pub struct CacheConfig {
        pub enabled: bool,
        pub ttl: Duration,
        pub max_entries: usize,
    }

    impl Default for CacheConfig {
        fn default() -> Self {
            Self {
                enabled: false,  // Disabled by default
                ttl: Duration::from_secs(300), // 5 minutes
                max_entries: 1000,
            }
        }
    }

    #[derive(Debug, Clone)]
    struct CacheEntry<T> {
        data: T,
        expires_at: Instant,
    }

    pub struct Cache<T> {
        entries: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
        config: CacheConfig,
    }

    impl<T: Clone> Cache<T> {
        pub fn new(config: CacheConfig) -> Self {
            Self {
                entries: Arc::new(RwLock::new(HashMap::new())),
                config,
            }
        }

        pub fn get(&self, key: &str) -> Option<T> {
            if !self.config.enabled {
                return None;
            }

            let entries = self.entries.read().ok()?;
            let entry = entries.get(key)?;

            if entry.expires_at > Instant::now() {
                Some(entry.data.clone())
            } else {
                None
            }
        }

        pub fn set(&self, key: String, value: T) {
            if !self.config.enabled {
                return;
            }

            let mut entries = match self.entries.write() {
                Ok(entries) => entries,
                Err(_) => return,
            };

            // Evict expired entries if at capacity
            if entries.len() >= self.config.max_entries {
                let now = Instant::now();
                entries.retain(|_, entry| entry.expires_at > now);

                // If still at capacity, remove oldest entries
                if entries.len() >= self.config.max_entries {
                    let to_remove = entries.len() - self.config.max_entries + 1;
                    let mut keys_to_remove: Vec<_> = entries.keys().take(to_remove).cloned().collect();
                    for key in keys_to_remove {
                        entries.remove(&key);
                    }
                }
            }

            entries.insert(key, CacheEntry {
                data: value,
                expires_at: Instant::now() + self.config.ttl,
            });
        }
    }
    ```

  - **Add performance optimizations in `src/utils.rs`**:

    ```rust
    use serde_json::Value;
    use std::io::Write;

    /// Fast JSON serialization with pre-allocated buffer
    pub fn serialize_json_fast<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
        let mut buf = Vec::with_capacity(1024); // Pre-allocate reasonable size
        serde_json::to_writer(&mut buf, value)?;
        Ok(buf)
    }

    /// Streaming JSON deserialization for large responses
    pub fn deserialize_json_stream<T: serde::de::DeserializeOwned>(
        reader: impl std::io::Read,
    ) -> Result<T, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    /// Memory-efficient string building for URLs
    pub fn build_url_path(base: &str, segments: &[&str]) -> String {
        let total_len = base.len() + segments.iter().map(|s| s.len() + 1).sum::<usize>();
        let mut url = String::with_capacity(total_len);
        url.push_str(base);

        for segment in segments {
            if !url.ends_with('/') && !segment.starts_with('/') {
                url.push('/');
            }
            url.push_str(segment);
        }

        url
    }
    ```

  - _Requirements: 10.1, 10.2, 10.3, 10.4, 10.5_

- [ ] 13. Final integration testing and validation

  - **Create comprehensive integration test suite in `tests/full_integration.rs`**:

    ```rust
    //! Full integration tests against Pi Network APIs
    //!
    //! These tests require actual Pi Network API credentials and should be run
    //! in a controlled environment with test accounts.

    use pi_rust::{PiNetworkClient, Network, SendAssetsParams, PiError};
    use std::env;

    // Helper to check if integration tests should run
    fn should_run_integration_tests() -> bool {
        env::var("RUN_INTEGRATION_TESTS").unwrap_or_default() == "true"
    }

    // Helper to get test credentials
    fn get_test_credentials() -> Option<(String, String, String, String)> {
        let api_key = env::var("PI_TEST_API_KEY").ok()?;
        let access_token = env::var("PI_TEST_ACCESS_TOKEN").ok()?;
        let source_secret = env::var("PI_TEST_SOURCE_SECRET").ok()?;
        let dest_account = env::var("PI_TEST_DEST_ACCOUNT").ok()?;

        Some((api_key, access_token, source_secret, dest_account))
    }

    #[tokio::test]
    async fn test_full_payment_workflow_integration() {
        if !should_run_integration_tests() {
            println!("Skipping integration test - set RUN_INTEGRATION_TESTS=true to enable");
            return;
        }

        let (api_key, access_token, _, _) = match get_test_credentials() {
            Some(creds) => creds,
            None => {
                println!("Skipping integration test - missing credentials");
                return;
            }
        };

        let client = PiNetworkClient::new(api_key).unwrap();

        // Test user authentication
        let profile = client.get_user_profile(&access_token).await.unwrap();
        assert!(!profile.user.uid.is_empty());
        assert!(!profile.user.username.is_empty());

        // Note: Payment creation is handled client-side via Pi App Platform SDK
        // For integration tests, we would test with existing payment IDs

        // Test payment retrieval (would use actual payment ID from client-side creation)
        // let payment = client.get_payment("actual_payment_id").await.unwrap();
        // assert!(!payment.identifier.is_empty());

        // Test payment approval (would use actual payment ID)
        // let approved = client.approve_payment("actual_payment_id").await.unwrap();
        // assert!(approved.status.developer_approved);

        // Test payment completion (would use actual payment ID and transaction ID)
        // let completed = client.complete_payment("actual_payment_id", "actual_tx_id").await.unwrap();
        // assert!(completed.status.developer_completed);
    }

    #[tokio::test]
    async fn test_stellar_operations_integration() {
        if !should_run_integration_tests() {
            return;
        }

        let (api_key, _, source_secret, dest_account) = match get_test_credentials() {
            Some(creds) => creds,
            None => return,
        };

        let client = PiNetworkClient::new(api_key).unwrap();

        // Test balance checking
        let balance = client.get_account_balance(
            Network::PiTestnet,
            &dest_account,
        ).await.unwrap();

        assert!(balance >= 0.0);

        // Only test sending if we have sufficient balance
        if balance > 1.0 {
            let params = SendAssetsParams {
                network: Network::PiTestnet,
                source_secret,
                destination: dest_account,
                amount: 0.1, // Small test amount
                memo: Some("Integration test".to_string()),
                fee: None,
            };

            let result = client.send_native_assets(params).await.unwrap();
            assert!(!result.hash.is_empty());
            assert!(result.ledger > 0);
        }
    }

    #[tokio::test]
    async fn test_error_handling_integration() {
        if !should_run_integration_tests() {
            return;
        }

        let (api_key, _, _, _) = match get_test_credentials() {
            Some(creds) => creds,
            None => return,
        };

        let client = PiNetworkClient::new(api_key).unwrap();

        // Test invalid access token
        let result = client.get_user_profile("invalid-token").await;
        assert!(matches!(result, Err(PiError::Authentication(_))));

        // Test nonexistent payment
        let result = client.get_payment("nonexistent-payment-id").await;
        assert!(matches!(result, Err(PiError::PiNetwork { .. })));

        // Test invalid payment ID
        let result = client.get_payment("").await;
        assert!(matches!(result, Err(PiError::Configuration(_))));
    }
    ```

  - **Create performance validation script in `scripts/validate_performance.sh`**:

    ```bash
    #!/bin/bash
    set -e

    echo "🚀 Running performance validation..."

    # Run benchmarks
    echo "Running benchmarks..."
    cargo bench --all-features

    # Check binary size
    echo "Checking binary size..."
    cargo build --release
    BINARY_SIZE=$(stat -c%s "target/release/deps/pi_rust-"* | head -1)
    echo "Binary size: $BINARY_SIZE bytes"

    # Memory usage test
    echo "Testing memory usage..."
    cargo test --release test_memory_usage -- --ignored

    # Concurrent request test
    echo "Testing concurrent performance..."
    cargo test --release test_concurrent_performance -- --ignored

    echo "✅ Performance validation completed"
    ```

  - **Create security audit script in `scripts/security_audit.sh`**:

    ```bash
    #!/bin/bash
    set -e

    echo "🔒 Running security audit..."

    # Check for known vulnerabilities
    echo "Checking for known vulnerabilities..."
    cargo audit

    # Check for unsafe code
    echo "Checking for unsafe code..."
    if grep -r "unsafe" src/; then
        echo "⚠️  Found unsafe code - please review"
    else
        echo "✅ No unsafe code found"
    fi

    # Check for hardcoded secrets
    echo "Checking for hardcoded secrets..."
    if grep -r -i "password\|secret\|key\|token" src/ --include="*.rs" | grep -v "// " | grep -v "pub "; then
        echo "⚠️  Potential hardcoded secrets found - please review"
    else
        echo "✅ No hardcoded secrets found"
    fi

    # Check dependencies
    echo "Checking dependency licenses..."
    cargo license

    echo "✅ Security audit completed"
    ```

  - _Requirements: 6.5, 9.5, 10.5_
