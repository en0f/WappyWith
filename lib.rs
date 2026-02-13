pub mod technologies;
pub mod detector;

use anyhow::{Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use technologies::Technology;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub name: String,
    pub category: String,
    pub version: Option<String>,
    pub website: Option<String>,
    pub confidence: u8,
    pub matched_patterns: Vec<String>,
}

pub struct Analyzer {
    technologies: Vec<Technology>,
}

pub struct WebPage {
    pub url: String,
    pub html: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub scripts: Vec<String>,
    pub meta_tags: HashMap<String, String>,
}

impl Analyzer {
    pub fn new() -> Result<Self> {
        let technologies = technologies::load_technologies()?;
        Ok(Self { technologies })
    }

    pub fn analyze(&self, url: &str) -> Result<Vec<DetectionResult>> {
        let webpage = self.fetch_webpage(url)?;
        let mut results = Vec::new();

        for tech in &self.technologies {
            if let Some(result) = self.detect_technology(tech, &webpage) {
                results.push(result);
            }
        }

        // Sort by confidence
        results.sort_by(|a, b| b.confidence.cmp(&a.confidence));

        Ok(results)
    }

    fn fetch_webpage(&self, url: &str) -> Result<WebPage> {
        let response = ureq::get(url)
            .set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .call()
            .context("Failed to fetch URL")?;

        // Extract headers
        let mut headers = HashMap::new();
        for name in response.headers_names() {
            if let Some(value) = response.header(&name) {
                headers.insert(name.to_lowercase(), value.to_string());
            }
        }

        let html = response.into_string()?;
        let document = Html::parse_document(&html);

        // Extract scripts
        let script_selector = Selector::parse("script[src]").unwrap();
        let scripts: Vec<String> = document
            .select(&script_selector)
            .filter_map(|el| el.value().attr("src"))
            .map(String::from)
            .collect();

        // Extract meta tags
        let meta_selector = Selector::parse("meta").unwrap();
        let mut meta_tags = HashMap::new();
        for el in document.select(&meta_selector) {
            if let Some(name) = el.value().attr("name") {
                if let Some(content) = el.value().attr("content") {
                    meta_tags.insert(name.to_string(), content.to_string());
                }
            }
        }

        // Cookies (simplified)
        let cookies = HashMap::new();

        Ok(WebPage {
            url: url.to_string(),
            html,
            headers,
            cookies,
            scripts,
            meta_tags,
        })
    }

    fn detect_technology(&self, tech: &Technology, webpage: &WebPage) -> Option<DetectionResult> {
        let mut matched_patterns = Vec::new();
        let mut confidence = 0u8;
        let mut version = None;

        // Check HTML patterns
        if let Some(patterns) = &tech.html {
            for pattern in patterns {
                if detector::check_pattern(&webpage.html, pattern) {
                    matched_patterns.push(format!("HTML: {}", pattern.pattern));
                    confidence += 20;
                    if version.is_none() {
                        version = detector::extract_version(&webpage.html, pattern);
                    }
                }
            }
        }

        // Check script patterns
        if let Some(patterns) = &tech.scripts {
            for pattern in patterns {
                for script in &webpage.scripts {
                    if detector::check_pattern(script, pattern) {
                        matched_patterns.push(format!("Script: {}", pattern.pattern));
                        confidence += 25;
                        if version.is_none() {
                            version = detector::extract_version(script, pattern);
                        }
                    }
                }
            }
        }

        // Check headers
        if let Some(headers) = &tech.headers {
            for (header_name, pattern) in headers {
                let header_key = header_name.to_lowercase();
                if let Some(header_value) = webpage.headers.get(&header_key) {
                    if detector::check_pattern(header_value, pattern) {
                        matched_patterns.push(format!("Header: {}", header_name));
                        confidence += 30;
                        if version.is_none() {
                            version = detector::extract_version(header_value, pattern);
                        }
                    }
                }
            }
        }

        // Check meta tags
        if let Some(meta) = &tech.meta {
            for (meta_name, pattern) in meta {
                if let Some(meta_value) = webpage.meta_tags.get(meta_name) {
                    if detector::check_pattern(meta_value, pattern) {
                        matched_patterns.push(format!("Meta: {}", meta_name));
                        confidence += 25;
                        if version.is_none() {
                            version = detector::extract_version(meta_value, pattern);
                        }
                    }
                }
            }
        }

        // Check cookies
        if let Some(cookies) = &tech.cookies {
            for (cookie_name, pattern) in cookies {
                if let Some(cookie_value) = webpage.cookies.get(cookie_name) {
                    if detector::check_pattern(cookie_value, pattern) {
                        matched_patterns.push(format!("Cookie: {}", cookie_name));
                        confidence += 25;
                    }
                }
            }
        }

        if !matched_patterns.is_empty() {
            // Cap confidence at 100
            confidence = confidence.min(100);

            Some(DetectionResult {
                name: tech.name.clone(),
                category: tech.category.clone(),
                version,
                website: tech.website.clone(),
                confidence,
                matched_patterns,
            })
        } else {
            None
        }
    }
}
