# Browser Automation Library

A modular, Rust-based browser automation library leveraging the **Fantoccini** WebDriver client. This library provides an extensible framework for automating browser actions, interacting with web elements, and executing JavaScript.

## Features

- **Session Management**: Start, navigate, and close browser sessions.
- **Element Interaction**: Click, type text, retrieve attributes, and manipulate DOM elements.
- **Utility Functions**:
  - Retry logic with exponential backoff.
  - Random delays to simulate human-like interaction.
  - Screenshot capturing.
  - JavaScript execution.
- **Modular Design**: Easily extend functionality by adding new modules.

## Requirements

- Rust (Edition 2021 or later)
- Chromedriver (included in the repository for Windows)
- WebDriver server running locally (`http://localhost:9515`)

## Installation

Add this library as a dependency in your `Cargo.toml`:

```toml
[dependencies]
browser_automation = "0.1.6"
```

## Getting Started

### Example: Automate Login on LinkedIn

```rust
use browser_automation::session::session::BrowserSession;

#[tokio::main]
async fn main() {
    let mut session = BrowserSession::new("https://www.linkedin.com/login").await.unwrap();

    let username_field = session.find_element("input#username").await.unwrap();
    username_field.type_text("your_email@example.com").await.unwrap();

    let password_field = session.find_element("input#password").await.unwrap();
    password_field.type_text("your_password").await.unwrap();

    let sign_in_button = session.find_element("button.btn__primary--large").await.unwrap();
    sign_in_button.click().await.unwrap();

    session.close().await.unwrap();
}
```

## Directory Structure

- **src/lib.rs**: Entry point, exposes modules.
- **src/elements/**: Functions to interact with DOM elements.
- **src/session/**: Browser session management.
- **src/utils/**: Utility functions for retries, delays, screenshots, etc.
- **chromedriver-win64/**: Pre-configured Chromedriver binary.

## Dependencies

- [Fantoccini](https://crates.io/crates/fantoccini) for WebDriver-based browser control.
- [Tokio](https://crates.io/crates/tokio) for async programming.
- [Serde](https://crates.io/crates/serde) for JSON serialization/deserialization.
- [Rand](https://crates.io/crates/rand) for random delays.

## Contributing

Contributions are welcome! Please create issues or submit pull requests to the [GitHub repository](https://github.com/harshapalnati/browser_automation_lib).

## License

This project is licensed under the MIT License.

