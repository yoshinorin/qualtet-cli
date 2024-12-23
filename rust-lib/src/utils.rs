use glob_match::glob_match;

pub fn remove_template_engines_syntax(text: &str) -> String {
  text.replace("{% raw %}", "").replace("{% endraw %}", "")
}

pub fn format_path(path: &str, content_type: &str) -> String {
  let mut p: String = if path.ends_with('/') {
    path.to_string()
  } else {
    format!("{}/", path)
  };

  if let Some(index) = p.rfind(".html/") {
    let prefix = &p[..index];
    if let Some(slash_index) = prefix.rfind('/') {
      p = format!("{}", &prefix[..=slash_index]);
    }
  }

  if !p.starts_with('/') {
    p = format!("/{}", p);
  }
  if content_type == "article" && !p.contains("/articles") {
    p = format!("/articles{}", p);
  }
  p
}

pub fn should_skip_paths(path: &str, skip_paths: &[&str]) -> bool {
  for skip_path in skip_paths {
    if glob_match(skip_path, path) {
      return true;
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_template_engines_syntax() {
    let input = "{% raw %}Hello{% endraw %}";
    let expected = "Hello";
    assert_eq!(remove_template_engines_syntax(input), expected);
  }

  #[test]
  fn test_remove_template_engines_syntax_multiple() {
    let input = "{% raw %}Hello{% endraw %} World {% raw %}Rust{% endraw %}";
    let expected = "Hello World Rust";
    assert_eq!(remove_template_engines_syntax(input), expected);
  }

  #[test]
  fn test_remove_template_engines_syntax_no_tags() {
    let input = "Hello World";
    let expected = "Hello World";
    assert_eq!(remove_template_engines_syntax(input), expected);
  }

  #[test]
  fn test_remove_template_engines_syntax_empty() {
    let input = "";
    let expected = "";
    assert_eq!(remove_template_engines_syntax(input), expected);
  }

  #[test]
  fn test_remove_template_engines_syntax_only_tags() {
    let input = "{% raw %}{% endraw %}";
    let expected = "";
    assert_eq!(remove_template_engines_syntax(input), expected);
  }

  #[test]
  fn test_format_path() {
    assert_eq!(
      format_path("path/to/resource", "article"),
      "/articles/path/to/resource/"
    );
    assert_eq!(
      format_path("path/to/resource/index.html", "article"),
      "/articles/path/to/resource/"
    );
    assert_eq!(
      format_path("path/to/resource/index.html/", "article"),
      "/articles/path/to/resource/"
    );
    assert_eq!(
      format_path("path/to/resource/", "article"),
      "/articles/path/to/resource/"
    );
    assert_eq!(
      format_path("/path/to/resource", "article"),
      "/articles/path/to/resource/"
    );
    assert_eq!(
      format_path("/path/to/resource/", "article"),
      "/articles/path/to/resource/"
    );
    assert_eq!(
      format_path("path/to/resource", "other"),
      "/path/to/resource/"
    );
    assert_eq!(
      format_path("path/to/resource/", "other"),
      "/path/to/resource/"
    );
    assert_eq!(
      format_path("/path/to/resource", "other"),
      "/path/to/resource/"
    );
    assert_eq!(
      format_path("/path/to/resource/", "other"),
      "/path/to/resource/"
    );
  }

  #[test]
  fn test_should_skip_paths() {
    let skip_paths = [
      "temp/**",
      "temp/**/hoge.md",
      "_drafts/**",
      "*.tmp",
      "**/temp",
    ];

    assert!(should_skip_paths("temp/some-path", &skip_paths));
    assert!(should_skip_paths("temp/foo/hoge.md", &skip_paths));
    assert!(should_skip_paths("_drafts/some-path", &skip_paths));
    assert!(should_skip_paths("_drafts/some-path/bar.md", &skip_paths));
    assert!(should_skip_paths("some-file.tmp", &skip_paths));
    assert!(should_skip_paths("some/path/temp", &skip_paths));
    assert!(!should_skip_paths("some-other-path", &skip_paths));
  }
}
