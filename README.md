# Pi Network Rust SDK

> **⚠️ Project Status: In Development**  
> This project is currently in the planning and specification phase. We are seeking contributors to help build this SDK. No implementation has been completed yet.

A comprehensive, type-safe, and async-first Rust SDK for integrating with Pi Network APIs. This SDK will enable server-side applications to manage payments, authenticate users, and interact with the Stellar blockchain that powers Pi Network.

## Planned Features

- **User Authentication** - Validate access tokens and retrieve user profiles
- **Payment Management** - Create, approve, cancel, and complete payments with full lifecycle support
- **Stellar Integration** - Send native Pi assets and query account balances across networks
- **Async/Await Support** - Built on tokio for high-performance concurrent operations
- **Type Safety** - Strongly-typed APIs with comprehensive error handling using `thiserror`
- **Well Tested** - Extensive unit and integration test coverage with mock servers
- **Rich Documentation** - Comprehensive rustdoc with examples and guides
- **Performance Optimized** - Connection pooling, retry logic, and efficient serialization
- **Configurable** - Flexible configuration with builder patterns and sensible defaults

## Project Vision

This SDK aims to provide a comprehensive Rust interface for Pi Network APIs, based on the existing [official C# SDK](https://github.com/pi-apps/pi-platform-docs/tree/master/SDK) developed by the Pi Network team. The goal is to bring the same functionality to the Rust ecosystem with idiomatic Rust patterns and modern async/await support.

## Project Documentation

### Specification Documents

- [Requirements Document](.specs/requirements.md) - Detailed requirements and acceptance criteria
- [Design Document](.specs/design.md) - Architecture and technical design
- [Implementation Tasks](.specs/tasks.md) - Detailed task breakdown for contributors

### Reference Materials

- [Pi Network C# SDK](https://github.com/pi-apps/pi-platform-docs/tree/master/SDK) - Official reference implementation
- [Pi Platform APIs Documentation](https://pi-platform-docs.vercel.app/) - Official API documentation
- **API Base URL**: `https://api.minepi.com/v2`
- **Supported Endpoints**:
  - `GET /me` - Get user information (requires Bearer token)
  - `GET /payments/{payment_id}` - Get payment details (requires API key)
  - `POST /payments/{payment_id}/approve` - Approve payment (requires API key)
  - `POST /payments/{payment_id}/complete` - Complete payment (requires API key)

## Technical Approach

The SDK will be built using modern Rust practices and will include:

- **Async-first design** using tokio for non-blocking I/O operations
- **Type-safe APIs** leveraging Rust's type system to prevent runtime errors
- **Comprehensive error handling** using `thiserror` for clear error propagation
- **Modular architecture** with clean separation between authentication, payments, and Stellar operations
- **Extensive testing** including unit tests, integration tests, and benchmarks
- **Rich documentation** with rustdoc comments and practical examples

## Planned Network Support

The SDK will support multiple Pi Network and Stellar networks:

- **Pi Mainnet** - Production Pi Network
- **Pi Testnet** - Pi Network testing environment
- **Stellar Testnet** - Stellar testing network for development

## Contributing

**We need your help!** This project is in the early stages and we're looking for contributors to help build this SDK.

### How to Get Started

1. **Review the specifications**:

   - Read the [Requirements Document](.specs/requirements.md)
   - Study the [Design Document](.specs/design.md)
   - Check the [Implementation Tasks](.specs/tasks.md)

2. **Set up your development environment**:

   - Install Rust (1.70 or later)
   - Install required tools: `rustup component add clippy rustfmt`
   - Fork and clone this repository

3. **Pick a task**:
   - Browse the [task list](.specs/tasks.md)
   - Look for issues labeled with different skill levels
   - Start with foundational tasks like project setup or data models

### Contribution Areas

We need help with:

- **Project Setup** - Initial Cargo.toml, module structure, and CI/CD
- **Data Models** - Implementing the Pi Network API data structures
- **HTTP Client** - Building the core client with proper error handling
- **Authentication** - User profile and token validation
- **Payment Operations** - Full payment lifecycle impleme
  We have issues labeled for different skill levels:

- 🟢 **good first issue** - Perfect for newcomers to Rust or the project
- 🟡 **help wanted** - Features and improvements we'd love help with
- 🔴 **advanced** - Complex features requiring deep Rust knowledge

See our [Contributing Guide](CONTRIBUTING.md) for detailed information.

## 📊 Project Status

This project is actively developed and maintained. Current status:

- 🚧 **Core API Implementation** - Payment and auth operations
- 🚧 **Stellar Integration** - Basic blockchain operations
- 🚧 **Comprehensive Testing** - Unit and integration tests
- 🚧 **Documentation** - API docs and guides
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

- 🚧 Core payment operations
- 🚧 User authentication
- 🚧 Basic Stellar integration
- 🚧 Comprehensive error handling

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
