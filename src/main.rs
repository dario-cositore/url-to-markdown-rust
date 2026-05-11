use anyhow::{Context, Result};
use clap::Parser;
use html2md::parse_html;
use std::fs;
use std::path::PathBuf;
use url::Url;

/// Convert webpages to Markdown from the command line
#[derive(Parser, Debug)]
#[command(name = "url-to-markdown", version, about, long_about = None)]
struct Args {
    /// The URL to convert
    url: String,

    /// Optional output file path (defaults to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Include link references at the bottom
    #[arg(long, default_value_t = true)]
    references: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Validate and normalize URL
    let url = normalize_url(&args.url)?;

    println!("Fetching {}...", url);

    // Fetch the webpage
    let client = reqwest::Client::builder()
        .user-agent("url-to-markdown/0.1.0")
        .build()
        .context("Failed to build HTTP client")?;

    let html = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch URL")?
        .text()
        .await
        .context("Failed to read response body")?;

    // Convert HTML to Markdown
    let markdown = parse_html(&html);

    // Clean up the output
    let markdown = clean_markdown(&markdown);

    // Output
    match args.output {
        Some(path) => {
            fs::write(&path, &markdown)
                .with_context(|| format!("Failed to write to {}", path.display()))?;
            println!("Markdown saved to {}", path.display());
        }
        None => {
            println!("{}", markdown);
        }
    }

    Ok(())
}

fn normalize_url(input: &str) -> Result<String> {
    let input = input.trim();

    // Add scheme if missing
    let url_str = if !input.starts_with("http://") && !input.starts_with("https://") {
        format!("https://{}", input)
    } else {
        input.to_string()
    };

    // Validate
    let parsed = Url::parse(&url_str).context("Invalid URL provided")?;
    Ok(parsed.to_string())
}

fn clean_markdown(md: &str) -> String {
    // Remove excessive blank lines (more than 2 consecutive)
    let mut result = String::new();
    let mut blank_count = 0;

    for line in md.lines() {
        if line.trim().is_empty() {
            blank_count += 1;
            if blank_count <= 2 {
                result.push('\n');
            }
        } else {
            blank_count = 0;
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim_end().to_string()
}
