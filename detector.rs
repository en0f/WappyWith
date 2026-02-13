use crate::technologies::Pattern;
use regex::Regex;

pub fn check_pattern(text: &str, pattern: &Pattern) -> bool {
    match Regex::new(&pattern.pattern) {
        Ok(re) => re.is_match(text),
        Err(_) => {
            // If regex is invalid, fall back to simple string matching
            text.contains(&pattern.pattern)
        }
    }
}

pub fn extract_version(text: &str, pattern: &Pattern) -> Option<String> {
    if pattern.version.is_none() {
        return None;
    }

    let re = Regex::new(&pattern.pattern).ok()?;
    let captures = re.captures(text)?;

    if captures.len() < 2 {
        return None;
    }

    // Get the first capture group (version number)
    captures.get(1).map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pattern_simple() {
        let pattern = Pattern::new("wordpress");
        assert!(check_pattern("This is wordpress content", &pattern));
        assert!(!check_pattern("This is drupal content", &pattern));
    }

    #[test]
    fn test_check_pattern_regex() {
        let pattern = Pattern::new(r"/wp-content/");
        assert!(check_pattern("/wp-content/themes/style.css", &pattern));
        assert!(!check_pattern("/themes/style.css", &pattern));
    }

    #[test]
    fn test_extract_version() {
        let pattern = Pattern::with_version(r"WordPress ([0-9.]+)", "$1");
        let version = extract_version("WordPress 6.4.2", &pattern);
        assert_eq!(version, Some("6.4.2".to_string()));
    }

    #[test]
    fn test_extract_version_no_match() {
        let pattern = Pattern::with_version(r"WordPress ([0-9.]+)", "$1");
        let version = extract_version("Drupal 10.0", &pattern);
        assert_eq!(version, None);
    }
}
