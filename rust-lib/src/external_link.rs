use regex::Regex;
use std::sync::LazyLock;
use url::Url;

static A_TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r#"<a(?:\s+?|\s+?[^<>]+?\s+?)href=["']((?:https?:|//)[^<>"']+)["'][^<>]*>"#).unwrap()
});

static TARGET_ATTR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"target=").unwrap());

static REL_ATTR_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"rel=").unwrap());

static REL_STR_ATTR_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r#"rel=["']([^<>"']*)["']"#).unwrap());

/// Check if a URL is an external link
fn is_external_link(href: &str, base_url: &str) -> bool {
  // Handle protocol-relative URLs
  let normalized_href = if href.starts_with("//") {
    format!("https:{}", href)
  } else {
    href.to_string()
  };

  // Parse URLs
  let parsed_href = match Url::parse(&normalized_href) {
    Ok(url) => url,
    Err(_) => return false,
  };

  let parsed_base = match Url::parse(base_url) {
    Ok(url) => url,
    Err(_) => return true, // If base URL is invalid, treat as external
  };

  // Compare hosts
  match (parsed_href.host_str(), parsed_base.host_str()) {
    (Some(href_host), Some(base_host)) => href_host != base_host,
    _ => true,
  }
}

/// Replace external links with appropriate attributes
pub fn replace_external_link(data: &str, base_url: &str) -> String {
  A_TAG_REGEX
    .replace_all(data, |caps: &regex::Captures| {
      let full_match = caps.get(0).unwrap().as_str();
      let href = caps.get(1).unwrap().as_str();

      // Skip if not external link or already has target attribute
      if !is_external_link(href, base_url) || TARGET_ATTR_REGEX.is_match(full_match) {
        return full_match.to_string();
      }

      // Check if rel attribute exists
      if REL_ATTR_REGEX.is_match(full_match) {
        // Update existing rel attribute first
        let updated_rel = REL_STR_ATTR_REGEX.replace(full_match, |rel_caps: &regex::Captures| {
          let rel_value = rel_caps.get(1).unwrap().as_str();
          if rel_value.contains("noopener") {
            rel_caps.get(0).unwrap().as_str().to_string()
          } else {
            format!(r#"rel="{} noopener""#, rel_value)
          }
        });

        // Add target attribute before href
        updated_rel.replace("href=", r#"target="_blank" href="#)
      } else {
        // Add both target and rel attributes before href
        full_match.replace(
          "href=",
          r#"target="_blank" rel="noopener external nofollow noreferrer" href="#,
        )
      }
    })
    .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_external_link() {
    let base_url = "https://example.com";

    // External links
    assert!(is_external_link("https://example.org/test", base_url));
    assert!(is_external_link("//example.net/test", base_url));
    assert!(is_external_link("https://sub.example.org", base_url));

    // Internal links
    assert!(!is_external_link("https://example.com/page", base_url));
    assert!(!is_external_link("https://example.com", base_url));
  }

  #[test]
  fn test_relative_paths() {
    let base_url = "https://example.com";

    // Test relative path (should not modify - not matched by regex)
    let input = r#"<a href="/page">Relative</a>"#;
    assert_eq!(replace_external_link(input, base_url), input);

    // Test relative path with directory (should not modify - not matched by regex)
    let input = r#"<a href="../page">Relative Up</a>"#;
    assert_eq!(replace_external_link(input, base_url), input);

    // Test anchor link (should not modify - not matched by regex)
    let input = r##"<a href="#section">Anchor</a>"##;
    assert_eq!(replace_external_link(input, base_url), input);

    // Test query string relative (should not modify - not matched by regex)
    let input = r#"<a href="?page=1">Query</a>"#;
    assert_eq!(replace_external_link(input, base_url), input);
  }

  #[test]
  fn test_external_link_replacement() {
    let base_url = "https://example.com";

    // Test external link without attributes
    let input = r#"<a href="https://example.org">Example Org</a>"#;
    let expected = r#"<a target="_blank" rel="noopener external nofollow noreferrer" href="https://example.org">Example Org</a>"#;
    assert_eq!(replace_external_link(input, base_url), expected);

    // Test external link with existing target (should not modify)
    let input = r#"<a href="https://example.org" target="_self">Example Org</a>"#;
    assert_eq!(replace_external_link(input, base_url), input);

    // Test external link with existing rel attribute
    let input = r#"<a href="https://example.org" rel="bookmark">Example Org</a>"#;
    let expected =
      r#"<a target="_blank" href="https://example.org" rel="bookmark noopener">Example Org</a>"#;
    assert_eq!(replace_external_link(input, base_url), expected);

    // Test internal link (should not modify)
    let input = r#"<a href="https://example.com/page">Internal</a>"#;
    assert_eq!(replace_external_link(input, base_url), input);

    // Test protocol-relative external link
    let input = r#"<a href="//example.net/test">Example Net</a>"#;
    let expected = r#"<a target="_blank" rel="noopener external nofollow noreferrer" href="//example.net/test">Example Net</a>"#;
    assert_eq!(replace_external_link(input, base_url), expected);
  }

  #[test]
  fn test_existing_rel_with_noopener() {
    let base_url = "https://example.com";
    let input = r#"<a href="https://example.org" rel="noopener bookmark">Example Org</a>"#;
    let expected =
      r#"<a target="_blank" href="https://example.org" rel="noopener bookmark">Example Org</a>"#;
    assert_eq!(replace_external_link(input, base_url), expected);
  }
}
