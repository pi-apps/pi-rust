# Pi Network Rust SDK

[![Crates.io](https://img.shields.io/crates/v/pi-rust.svg)](https://crates.io/crates/pi-rust)
[![Documentation](https://docs.rs/pi-rust/badge.svg)](https://docs.rs/pi-rust)
[![Build Status](https://github.com/username/pi-rust/workflows/CI/badge.svg)](https://github.com/username/pi-rust/actions)
[![Coverage Status](https://codecov.io/gh/username/pi-rust/branch/main/graph/badge.svg)](https://codecov.io/gh/username/pi-rust)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/username/pi-rust#license)

A comprehensive, type-safe, and async-first Rust SDK for integrating with Pi Network APIs. This SDK enables server-side applications to manage payments, authenticate users, and interact with the Stellar blockchain that powers Pi Network.

## ✨ Features

- 🔐 **User Authentication** - Validate access tokens and retrieve user profiles
- 💰 **Payment Management** - Create, approve, cancel, and complete payments with full lifecycle support
- 🌟 **Stellar Integration** - Send native Pi assets and query account balances across networks
- 🔄 **Async/Await Support** - Built on tokio for high-performance concurrent operations
- 🛡️ **Type Safety** - Strongly-typed APIs with comprehensive error handling using `thiserror`
- 🧪 **Well Tested** - Extensive unit and integration test coverage with mock servers
- 📚 **Rich Documentation** - Comprehensive rustdoc with examples and guides
- 🚀 **Performance Optimized** - Connection pooling, retry logic, and efficient serialization
- 🔧 **Configurable** - Flexible configuration with builder patterns and sensible defaults

## 🚀 Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pi-rust = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"  # For metadata handling
```

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
        10.5,                                    // amount in Pi
        Some("Coffee purchase".to_string()),     // memo
        None,                                    // metadata
        profile.user.uid,                        // user ID
    ).await?;
    
    println!("Payment created: {}", payment.identifier);
    
    // Approve the payment
    let approved = client.approve_payment(&payment.identifier).await?;
    println!("Payment approved: {}", approved.status.developer_approved);
    
    // Check account balance on Pi Testnet
    let balance = client.get_account_balance(
        Network::PiTestnet,
        "GTEST123..."  // account ID
    ).await?;
    println!("Account balance: {} Pi", balance);
    
    Ok(())
}
```

## 📖 Documentation

### API Reference
- [Full API Documentation](https://docs.rs/pi-rust) - Complete rustdoc documentation
- [Examples](examples/) - Practical usage examples for common scenarios

### Guides
- [Getting Started Guide](docs/getting-started.md) - Step-by-step setup and basic usage
- [Payment Integration Guide](docs/payment-integration.md) - Complete payment workflow implementation
- [Stellar Operations Guide](docs/stellar-operations.md) - Blockchain operations and network handling
- [Error Handling Guide](docs/error-handling.md) - Best practices for error management
- [Configuration Guide](docs/configuration.md) - Advanced client configuration options

## 🔧 Configuration

### Basic Configuration

```rust
use pi_rust::PiNetworkClient;

let client = PiNetworkClient::new("your-api-key".to_string())?;
```

### Advanced Configuration

```rust
use pi_rust::{PiNetworkClient, ClientConfig};
use std::time::Duration;
use url::Url;

let config = ClientConfig::builder("your-api-key".to_string())
    .base_url(Url::parse("https://api.minepi.com/v2")?)
    .timeout(Duration::from_secs(30))
    .build();

let client = PiNetworkClient::with_config(config)?;
```

## 💡 Examples

### Complete Payment Workflow

```rust
use pi_rust::{PiNetworkClient, PiError};

async fn process_payment(
    client: &PiNetworkClient,
    access_token: &str,
    amount: f64,
) -> Result<(), PiError> {
    // 1. Authenticate user
    let profile = client.get_user_profile(access_token).await?;
    
    // 2. Create payment
    let payment = client.create_payment(
        amount,
        Some("Premium subscription".to_string()),
        Some(serde_json::json!({"product": "premium"})),
        profile.user.uid,
    ).await?;
    
    // 3. Approve payment
    let approved = client.approve_payment(&payment.identifier).await?;
    
    // 4. Wait for user completion in Pi app, then complete with tx ID
    // let completed = client.complete_payment(&payment.identifier, "tx_hash").await?;
    
    println!("Payment {} processed successfully", payment.identifier);
    Ok(())
}
```

### Stellar Operations

```rust
use pi_rust::{PiNetworkClient, Network, SendAssetsParams};

async fn send_pi_assets(client: &PiNetworkClient) -> Result<(), Box<dyn std::error::Error>> {
    // Check balance first
    let balance = client.get_account_balance(
        Network::PiTestnet,
        "GDEST123..."
    ).await?;
    
    if balance >= 10.0 {
        // Send Pi to another account
        let params = SendAssetsParams {
            network: Network::PiTestnet,
            source_secret: "SSOURCE123...".to_string(),
            destination: "GDEST456...".to_string(),
            amount: 5.0,
            memo: Some("Payment".to_string()),
            fee: None,
        };
        
        let result = client.send_native_assets(params).await?;
        println!("Transaction successful: {}", result.hash);
    }
    
    Ok(())
}
```

### Error Handling

```rust
use pi_rust::{PiNetworkClient, PiError};

async fn handle_errors(client: &PiNetworkClient) {
    match client.get_user_profile("invalid-token").await {
        Ok(profile) => println!("User: {}", profile.user.username),
        Err(PiError::Authentication(msg)) => {
            eprintln!("Authentication failed: {}", msg);
        }
        Err(PiError::PiNetwork { error_name, error_message, .. }) => {
            eprintln!("Pi Network error {}: {}", error_name, error_message);
        }
        Err(PiError::Http(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => {
            eprintln!("Other error: {}", e);
        }
    }
}
```

## 🌐 Supported Networks

The SDK supports multiple Pi Network and Stellar networks:

- **Pi Mainnet** - Production Pi Network (`Network::PiMainnet`)
- **Pi Testnet** - Pi Network testing environment (`Network::PiTestnet`)
- **Stellar Testnet** - Stellar testing network (`Network::StellarTestnet`)

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run integration tests (requires credentials)
RUN_INTEGRATION_TESTS=true cargo test --test integration_tests

# Run benchmarks
cargo bench
```

### Test Requirements

For integration tests, set these environment variables:

```bash
export PI_TEST_API_KEY="your-test-api-key"
export PI_TEST_ACCESS_TOKEN="user-access-token"
export PI_TEST_SOURCE_SECRET="stellar-secret-seed"
export PI_TEST_DEST_ACCOUNT="stellar-account-id"
export RUN_INTEGRATION_TESTS="true"
```

## 🤝 Contributing

We welcome contributions! This project is designed to be contributor-friendly with clear guidelines and comprehensive documentation.

### Getting Started

1. **Fork the repository** and clone your fork
2. **Install Rust** (1.70 or later) and required tools:
   ```bash
   rustup component add clippy rustfmt
   cargo install cargo-tarpaulin cargo-audit
   ```
3. **Run the test suite** to ensure everything works:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

### Development Workflow

1. **Create a feature branch** from `main`
2. **Implement your changes** following the coding standards
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Run the full test suite** and ensure all checks pass
6. **Submit a pull request** with a clear description

### Contribution Areas

We have issues labeled for different skill levels:

- 🟢 **good first issue** - Perfect for newcomers to Rust or the project
- 🟡 **help wanted** - Features and improvements we'd love help with
- 🔴 **advanced** - Complex features requiring deep Rust knowledge

See our [Contributing Guide](CONTRIBUTING.md) for detailed information.

## 📊 Project Status

This project is actively developed and maintained. Current status:

- ✅ **Core API Implementation** - Payment and auth operations
- ✅ **Stellar Integration** - Basic blockchain operations
- ✅ **Comprehensive Testing** - Unit and integration tests
- ✅ **Documentation** - API docs and guides
- 🚧 **Advanced Features** - Caching, connection pooling
- 🚧 **Performance Optimization** - Benchmarking and optimization
- 📋 **Planned** - WebSocket support, advanced retry strategies

## 🔒 Security

Security is a top priority. We:

- ✅ Run automated security audits with `cargo audit`
- ✅ Follow Rust security best practices
- ✅ Regularly update dependencies
- ✅ Provide secure examples and documentation

To report security vulnerabilities, please email [security@example.com](mailto:security@example.com).

## 📈 Performance

The SDK is designed for high performance:

- **Async/await throughout** - Non-blocking I/O operations
- **Connection pooling** - Efficient HTTP connection reuse
- **Configurable timeouts** - Prevent hanging requests
- **Retry logic** - Automatic retry with exponential backoff
- **Efficient serialization** - Optimized JSON processing

Benchmark results are available in the [benchmarks](benches/) directory.

## 🛣️ Roadmap

### Version 0.1.0 (Current)
- ✅ Core payment operations
- ✅ User authentication
- ✅ Basic Stellar integration
- ✅ Comprehensive error handling

### Version 0.2.0 (Planned)
- 🚧 WebSocket support for real-time updates
- 🚧 Advanced caching mechanisms
- 🚧 Enhanced retry strategies
- 🚧 Performance optimizations

### Version 0.3.0 (Future)
- 📋 Multi-signature transaction support
- 📋 Advanced Stellar operations
- 📋 Plugin system for extensions
- 📋 GraphQL API support

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🙏 Acknowledgments

- **Pi Network Team** - For creating the Pi Network and providing the C# SDK reference
- **Stellar Development Foundation** - For the Stellar blockchain infrastructure
- **Rust Community** - For the amazing ecosystem and tools
- **Contributors** - Everyone who helps make this project better

## 📞 Support

- 📖 **Documentation**: [docs.rs/pi-rust](https://docs.rs/pi-rust)
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/username/pi-rust/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/username/pi-rust/discussions)
- 📧 **Email**: [support@example.com](mailto:support@example.com)

---

<div align="center">

**[Documentation](https://docs.rs/pi-rust)** • 
**[Examples](examples/)** • 
**[Contributing](CONTRIBUTING.md)** • 
**[Changelog](CHANGELOG.md)**

Made with ❤️ by the Pi Network Rust community

</div>