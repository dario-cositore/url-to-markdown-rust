# URL to Markdown — Rust CLI

A fast, lightweight CLI tool that converts webpages to clean Markdown.

## Features

- **Fast** — Built with Rust for maximum performance
- **Clean output** — Strips navigation, scripts, and noise
- **Simple** — One command, any URL
- **Flexible** — Output to stdout or file

## Installation

### From source

```bash
git clone https://github.com/dariocositore/url-to-markdown-rust.git
cd url-to-markdown-rust
cargo build --release
```

The binary will be at `target/release/url-to-markdown`.

### Add to PATH

```bash
cargo install --path .
```

## Usage

```bash
# Convert a webpage and print to stdout
url-to-markdown https://example.com

# Save to file
url-to-markdown https://example.com -o output.md

# Works without scheme too
url-to-markdown example.com -o output.md
```

## Example

```bash
$ url-to-markdown https://example.com

# Example Domain

This domain is for use in illustrative examples in documents. You may use this
domain in literature without prior coordination or asking for permission.

[More information...](https://www.iana.org/domains/reserved)
```

## Dependencies

- [reqwest](https://github.com/seanmonstar/reqwest) — HTTP client
- [html2md](https://github.com/nickel-org/html2md) — HTML to Markdown conversion
- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [anyhow](https://github.com/dtolnay/anyhow) — Error handling

## License

MIT
