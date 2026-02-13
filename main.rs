use anyhow::Result;
use std::env;
use wappalyzer_rust::{Analyzer, DetectionResult};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let url = &args[1];
    let verbose = args.iter().any(|arg| arg == "--verbose" || arg == "-v");
    let json = args.iter().any(|arg| arg == "--json" || arg == "-j");

    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_usage();
        return Ok(());
    }

    println!("Analyzing: {}", url);
    println!();

    let analyzer = Analyzer::new()?;
    let results = analyzer.analyze(url)?;

    if json {
        print_json(&results)?;
    } else {
        print_results(&results, verbose);
    }

    Ok(())
}

fn print_usage() {
    println!("Wappalyzer-Rust - Detect technologies used on websites");
    println!();
    println!("Usage: wappalyzer-rust <URL> [OPTIONS]");
    println!();
    println!("Arguments:");
    println!("  <URL>              URL to analyze");
    println!();
    println!("Options:");
    println!("  -v, --verbose      Show detailed information");
    println!("  -j, --json         Output as JSON");
    println!("  -h, --help         Print help");
}

fn print_json(results: &[DetectionResult]) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(results)?);
    Ok(())
}

fn print_results(results: &[DetectionResult], verbose: bool) {
    if results.is_empty() {
        println!("No technologies detected.");
        return;
    }

    println!("Detected Technologies:");
    println!("{}", "=".repeat(50));

    for result in results {
        println!("\n{}", result.name);
        println!("  Category: {}", result.category);
        if let Some(version) = &result.version {
            println!("  Version: {}", version);
        }
        if let Some(website) = &result.website {
            println!("  Website: {}", website);
        }

        if verbose {
            println!("  Confidence: {}%", result.confidence);
            if !result.matched_patterns.is_empty() {
                println!("  Matched patterns:");
                for pattern in &result.matched_patterns {
                    println!("    - {}", pattern);
                }
            }
        }
    }
}
