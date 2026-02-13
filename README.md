# Wappalyzer-Rust

A Rust implementation of Wappalyzer - a technology profiler that identifies technologies used on websites.

## ⚠️ IMPORTANT: Rust Version Requirement

**This project requires Rust 1.83 or newer.**

The dependencies used by this project require modern Rust features. If you encounter build errors about "edition2024" or "rustc 1.83", you need to update Rust:

```bash
# Update Rust to the latest stable version
rustup update stable
rustup default stable

# Verify your Rust version
rustc --version
# Should show: rustc 1.83.0 or higher
```

## Features

- 🔍 **Comprehensive Technology Detection**: Identifies 100+ web technologies across 20+ categories:
  - **CMS**: WordPress, Drupal, Joomla, Wix, Squarespace, Webflow
  - **E-commerce**: Shopify, WooCommerce, Magento, BigCommerce, PrestaShop
  - **JavaScript Frameworks**: React, Vue.js, Angular, Svelte, Next.js, Nuxt.js, Gatsby, Ember.js
  - **JavaScript Libraries**: jQuery, jQuery UI
  - **CSS Frameworks**: Bootstrap, Tailwind CSS, Foundation, Bulma, Material-UI
  - **Analytics**: Google Analytics, Google Tag Manager, Adobe Analytics, Mixpanel, Segment, Hotjar
  - **Advertising**: Google Ads, Facebook Pixel
  - **CDNs**: Cloudflare, Fastly, Amazon CloudFront, Akamai
  - **Web Servers**: Nginx, Apache, Microsoft IIS, LiteSpeed
  - **Programming Languages**: PHP, Node.js, Python, ASP.NET
  - **Web Frameworks**: Ruby on Rails, Django, Express
  - **Payment Processors**: Stripe, PayPal, Square
  - **Live Chat**: Intercom, Zendesk, Drift, LiveChat, Tawk.to
  - **Video Players**: YouTube, Vimeo, Video.js
  - **Fonts**: Google Fonts, Font Awesome, Adobe Fonts
  - **Security**: reCAPTCHA, hCaptcha
  - **Marketing**: HubSpot, Mailchimp
  - **Hosting**: Vercel, Netlify, GitHub Pages
  - **Social**: Facebook SDK, Twitter Widgets, AddThis, ShareThis
  - And many more!

- 🎯 **Multiple Detection Methods**:
  - HTML content analysis
  - HTTP headers inspection
  - JavaScript file detection
  - Meta tag analysis
  - Cookie detection

- 📊 **Confidence Scoring**: Each detection includes a confidence score
- 🔢 **Version Detection**: Attempts to identify specific version numbers
- 💻 **Simple CLI**: Easy to use command-line tool
- 📝 **Multiple Output Formats**: Human-readable or JSON

## Installation

### 1. Install/Update Rust

```bash
# If you don't have Rust installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# If you already have Rust, update it:
rustup update stable
rustup default stable
```

### 2. Build the Project

```bash
cd wappalyzer-rust
cargo build --release
```

The first build will take a few minutes as it downloads and compiles dependencies.

## Usage

### Basic Analysis

```bash
# Using cargo run
cargo run -- https://example.com

# Or use the compiled binary
./target/release/wappalyzer-rust https://example.com
```

### Options

```bash
# Verbose output (shows confidence scores and matched patterns)
cargo run -- https://example.com --verbose

# JSON output
cargo run -- https://example.com --json

# Help
cargo run -- --help
```

## Example Output

```
Analyzing: https://wordpress.org

Detected Technologies:
==================================================

WordPress
  Category: CMS
  Website: https://wordpress.org

Nginx
  Category: Web Server
  Version: 1.24.0
  Website: https://nginx.org

Google Analytics
  Category: Analytics
  Website: https://analytics.google.com
```

## Troubleshooting

### "edition2024" or "rustc 1.83" Errors

This means your Rust version is too old. Update Rust:

```bash
rustup update stable
rustc --version  # Should be 1.83.0 or higher
```

### "no targets specified" Error

Make sure you're in the project directory:

```bash
cd wappalyzer-rust
ls -la  # Should show Cargo.toml and src/ directory
```

### SSL/Network Errors

Install required system dependencies:

```bash
# Ubuntu/Debian
sudo apt-get install libssl-dev pkg-config

# macOS
brew install openssl
```

### "src/lib.rs: No such file" Error

The src files are missing. Make sure you have:
- `src/main.rs`
- `src/lib.rs`
- `src/technologies.rs`
- `src/detector.rs`

Download the complete package again if files are missing.

## Project Structure

```
wappalyzer-rust/
├── Cargo.toml          # Project configuration and dependencies
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Core analyzer logic
│   ├── technologies.rs # Technology signature database
│   └── detector.rs     # Pattern matching engine
├── examples/
│   └── batch_analyze.rs
└── README.md
```

## How It Works

1. **Fetch**: Downloads the webpage and HTTP headers
2. **Parse**: Extracts scripts, meta tags, and other data
3. **Match**: Compares against technology signatures using regex
4. **Score**: Calculates confidence based on matches
5. **Extract**: Identifies version numbers when possible

## Extending

To add new technologies, edit `src/technologies.rs`:

```rust
Technology {
    name: "MyTech".to_string(),
    category: "Framework".to_string(),
    website: Some("https://mytech.com".to_string()),
    html: Some(vec![
        Pattern::new(r"mytech-pattern"),
    ]),
    scripts: Some(vec![
        Pattern::with_version(r"mytech-([0-9.]+)\.js", r"$1"),
    ]),
    headers: None,
    meta: None,
    cookies: None,
}
```

## Testing

```bash
cargo test
cargo test -- --nocapture  # With output
```

## Dependencies

- `ureq` - HTTP client (blocking, simple)
- `scraper` - HTML parsing
- `regex` - Pattern matching
- `serde` / `serde_json` - Serialization
- `anyhow` - Error handling

## Performance

- Single HTTP request per URL
- Fast regex-based pattern matching
- Minimal memory usage
- No JavaScript execution (static analysis only)

## Limitations

- Technology database is simplified vs. full Wappalyzer
- No JavaScript execution (can't detect dynamically loaded tech)
- Cookie detection limited to Set-Cookie headers
- Version detection depends on patterns in the HTML/headers

## Contributing

Contributions welcome:
- Add new technology signatures
- Improve version detection
- Add more detection methods
- Optimize performance

