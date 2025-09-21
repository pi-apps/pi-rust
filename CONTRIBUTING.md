# Contributing to Pi Network Rust SDK

Thank you for your interest in contributing to the Pi Network Rust SDK! This project is in its early development phase and we welcome contributions from developers of all skill levels.

## Project Status

**⚠️ Important**: This project is currently in the specification and planning phase. No implementation has been completed yet. We are actively seeking contributors to help build this SDK from the ground up.

## Getting Started

### Prerequisites

- **Rust**: Version 1.70 or later
- **Git**: For version control
- **Basic knowledge of**: Rust, async programming, HTTP APIs, and blockchain concepts (helpful but not required)

### Development Environment Setup

1. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install required tools**:
   ```bash
   rustup component add clippy rustfmt
   cargo install cargo-tarpaulin cargo-audit
   ```

3. **Fork and clone the repository**:
   ```bash
   git clone https://github.com/yourusername/pi-rust.git
   cd pi-rust
   ```

4. **Create a new branch for your work**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Understanding the Project

Before contributing, please familiarize yourself with:

1. **[Requirements Document](.kiro/specs/pi-rust-sdk/requirements.md)** - What the SDK needs to accomplish
2. **[Design Document](.kiro/specs/pi-rust-sdk/design.md)** - How the SDK will be architected
3. **[Implementation Tasks](.kiro/specs/pi-rust-sdk/tasks.md)** - Detailed breakdown of work to be done
4. **[Pi Network C# SDK](https://github.com/pi-apps/pi-platform-docs/tree/master/SDK)** - Reference implementation

## How to Contribute

### 1. Choose a Task

Browse the [Implementation Tasks](.kiro/specs/pi-rust-sdk/tasks.md) document to find something that matches your skill level:

- **Beginner-friendly tasks**:
  - Project setup (Cargo.toml, basic structure)
  - Data model implementation (structs with serde)
  - Basic unit tests
  - Documentation improvements

- **Intermediate tasks**:
  - HTTP client implementation
  - Error handling systems
  - Authentication modules
  - Payment operations

- **Advanced tasks**:
  - Stellar blockchain integration
  - Performance optimizations
  - Complex integration tests
  - CI/CD pipeline setup

### 2. Claim Your Task

1. Check if there's already an issue for the task
2. If not, create a new issue describing what you plan to work on
3. Comment on the issue to let others know you're working on it
4. Reference the specific task from the implementation plan

### 3. Implementation Guidelines

#### Code Style

- **Follow Rust conventions**: Use `cargo fmt` to format your code
- **Use clippy**: Run `cargo clippy` and fix all warnings
- **Naming**: Use clear, descriptive names for functions and variables
- **Comments**: Add rustdoc comments for all public APIs
- **Error handling**: Use `Result<T, E>` and proper error types

#### Code Structure

Follow the planned module structure from the design document:

```
src/
├── lib.rs              # Main library entry point
├── client.rs           # Core PiNetworkClient
├── config.rs           # Configuration and builder
├── models/             # Data models module
├── payments/           # Payment operations
├── auth/               # Authentication operations
├── stellar/            # Stellar blockchain operations
├── errors.rs           # Error types and handling
└── utils.rs            # Utility functions
```

#### Testing

- **Write tests** for all new functionality
- **Use descriptive test names** that explain what is being tested
- **Include both success and failure cases**
- **Use mock servers** for HTTP testing (wiremock crate)
- **Follow the testing patterns** shown in the task specifications

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name_success_case() {
        // Test implementation
    }
    
    #[test]
    fn test_function_name_error_case() {
        // Test error handling
    }
}
```

#### Documentation

- **Add rustdoc comments** for all public functions, structs, and modules
- **Include examples** in documentation when helpful
- **Update README.md** if you add new major functionality
- **Keep documentation up to date** with code changes

Example documentation:
```rust
/// Creates a new payment request
/// 
/// # Arguments
/// * `amount` - Payment amount in Pi
/// * `memo` - Optional memo for the payment
/// * `user_uid` - User ID from Pi Network authentication
/// 
/// # Returns
/// * `Result<Payment>` - The created payment or an error
/// 
/// # Example
/// ```rust
/// let payment = client.create_payment(10.0, Some("Coffee".to_string()), "user123".to_string()).await?;
/// ```
pub async fn create_payment(&self, amount: f64, memo: Option<String>, user_uid: String) -> Result<Payment> {
    // Implementation
}
```

### 4. Submission Process

1. **Ensure your code compiles**:
   ```bash
   cargo build
   ```

2. **Run tests**:
   ```bash
   cargo test
   ```

3. **Check formatting and linting**:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: implement payment creation functionality"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**:
   - Use a clear, descriptive title
   - Reference the issue you're addressing
   - Describe what you implemented and why
   - Include any testing notes or special considerations

## Pull Request Guidelines

### PR Title Format

Use conventional commit format:
- `feat: add payment creation functionality`
- `fix: resolve authentication token validation`
- `docs: update API documentation`
- `test: add integration tests for stellar operations`
- `refactor: improve error handling structure`

### PR Description Template

```markdown
## Description
Brief description of the changes in this PR.

## Related Issue
Closes #(issue number)

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Implementation Details
- Implemented X functionality
- Added Y tests
- Updated Z documentation

## Testing
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] I have tested the functionality manually (if applicable)

## Checklist
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
```

## Code Review Process

1. **Automated checks** will run on your PR (formatting, linting, tests)
2. **Maintainers will review** your code for:
   - Correctness and functionality
   - Code quality and style
   - Test coverage
   - Documentation completeness
3. **Address feedback** by making additional commits to your branch
4. **Once approved**, your PR will be merged

## Getting Help

### Communication Channels

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Pull Request Comments**: For code-specific questions

### Common Questions

**Q: I'm new to Rust. Can I still contribute?**
A: Absolutely! Start with simpler tasks like data models or documentation. The Rust community is very welcoming to beginners.

**Q: I don't understand blockchain/Pi Network. Can I help?**
A: Yes! Many tasks don't require deep blockchain knowledge. Focus on general Rust development tasks.

**Q: How do I test my changes without a Pi Network API key?**
A: Use the mock servers and unit tests. Integration tests will be set up later in the project.

**Q: What if I start working on something and get stuck?**
A: Don't hesitate to ask for help! Comment on the issue or create a discussion thread.

## Recognition

Contributors will be:
- Listed in the project's README
- Credited in release notes
- Given appropriate GitHub repository permissions based on contributions

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to the Pi Network Rust SDK! Your help is essential in making this project successful. 🚀