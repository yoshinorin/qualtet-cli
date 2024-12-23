pub fn remove_template_engines_syntax(text: &str) -> String {
  text.replace("{% raw %}", "").replace("{% endraw %}", "")
}

pub fn format_path(path: &str, content_type: &str) -> String {
  let mut p: String = if path.ends_with('/') {
    path.to_string()
  } else {
    format!("{}/", path)
  };

  if !p.starts_with('/') {
    p = format!("/{}", p);
  }
  if content_type == "article" && !p.contains("/articles") {
    p = format!("/articles{}", p);
  }
  p
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
}
