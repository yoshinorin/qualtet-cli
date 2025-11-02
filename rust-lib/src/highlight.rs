use serde::{Deserialize, Serialize};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Debug, Serialize, Deserialize)]
pub struct HighlightOptions {
  #[serde(default)]
  pub lang: Option<String>,
  #[serde(default)]
  pub caption: Option<String>,
  #[serde(default = "default_first_line")]
  pub first_line: i32,
}

fn default_first_line() -> i32 {
  1
}

fn normalize_language(lang: &str) -> &str {
  match lang {
    // Perl syntax is most similar to nginx config: block structures ({}), semicolons, and # comments
    "nginx" => "perl",
    "typescript" | "ts" | "tsx" => "javascript",
    _ => lang,
  }
}

mod syntax_highlighter {
  use super::*;
  use std::sync::LazyLock;

  static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);

  pub fn highlight_code_classed(code: &str, lang: &str) -> Result<String, String> {
    let normalized_lang = normalize_language(lang);

    let syntax = SYNTAX_SET
      .find_syntax_by_token(normalized_lang)
      .or_else(|| SYNTAX_SET.find_syntax_by_extension(normalized_lang))
      .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    let mut html_generator =
      ClassedHTMLGenerator::new_with_class_style(syntax, &SYNTAX_SET, ClassStyle::Spaced);

    for line in LinesWithEndings::from(code) {
      html_generator
        .parse_html_for_line_which_includes_newline(line)
        .map_err(|e| format!("Syntax highlighting error: {:?}", e))?;
    }

    Ok(html_generator.finalize())
  }
}

impl Default for HighlightOptions {
  fn default() -> Self {
    Self {
      lang: None,
      caption: None,
      first_line: 1,
    }
  }
}

pub fn highlight(code: &str, options: HighlightOptions) -> String {
  let lang = options.lang.as_deref().unwrap_or("plaintext");
  let highlighted_html = if lang != "plaintext" && !lang.is_empty() {
    syntax_highlighter::highlight_code_classed(code, lang).ok()
  } else {
    None
  };

  let lines: Vec<&str> = if let Some(ref html) = highlighted_html {
    html.split('\n').collect()
  } else {
    code.split('\n').collect()
  };

  let mut numbers = String::new();
  let mut content = String::new();

  for (i, line) in lines.iter().enumerate() {
    let line_number = options.first_line + i as i32;

    numbers.push_str(&format!("<span class=\"line\">{}</span><br>", line_number));

    let processed_line = if highlighted_html.is_none() {
      line
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
    } else {
      line.to_string()
    };

    content.push_str(&format!(
      "<span class=\"line\">{}</span><br>",
      processed_line
    ));
  }

  let caption_html = if let Some(caption) = &options.caption {
    format!("<figcaption>{}</figcaption>", caption)
  } else {
    String::new()
  };

  let mut result = format!("<figure class=\"highlight {}\">", lang);
  result.push_str(&caption_html);
  result.push_str("<table><tr>");
  result.push_str(&format!("<td class=\"gutter\"><pre>{}</pre></td>", numbers));
  result.push_str(&format!("<td class=\"code\"><pre>{}</pre></td>", content));
  result.push_str("</tr></table></figure>");

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_highlight_with_syntax() {
    let code = "console.log('hello');";
    let options = HighlightOptions {
      lang: Some("javascript".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<figure class=\"highlight javascript\">"));
    assert!(result.contains("<table>"));
    assert!(result.contains("<td class=\"gutter\">"));
    assert!(result.contains("<td class=\"code\">"));
    assert!(result.contains("<span class=\"line\">1</span>"));
    assert!(result.contains("<span class="));
  }

  #[test]
  fn test_plaintext_no_syntax() {
    let code = "hello world";
    let options = HighlightOptions {
      lang: Some("plaintext".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("hello world"));
    assert!(!result.contains("<span class=\"source"));
  }

  #[test]
  fn test_html_escape() {
    let code = "<script>alert('xss')</script>";
    let options = HighlightOptions {
      lang: Some("plaintext".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("&lt;script&gt;"));
    assert!(result.contains("&lt;/script&gt;"));
    assert!(!result.contains("<script>"));
  }

  #[test]
  fn test_gutter_option() {
    let code = "line1\nline2\nline3";
    let options = HighlightOptions {
      lang: Some("plaintext".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<td class=\"gutter\">"));
    assert!(result.contains("<span class=\"line\">1</span>"));
    assert!(result.contains("<span class=\"line\">2</span>"));
    assert!(result.contains("<span class=\"line\">3</span>"));
  }

  #[test]
  fn test_caption_with_wrap() {
    let code = "test";
    let options = HighlightOptions {
      lang: Some("javascript".to_string()),
      caption: Some("<span>Example</span>".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<figcaption><span>Example</span></figcaption>"));
  }

  #[test]
  fn test_first_line_number() {
    let code = "line1\nline2";
    let options = HighlightOptions {
      lang: Some("plaintext".to_string()),
      first_line: 10,
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<span class=\"line\">10</span>"));
    assert!(result.contains("<span class=\"line\">11</span>"));
    assert!(!result.contains("<span class=\"line\">1</span>"));
  }

  #[test]
  fn test_multiline_code() {
    let code = "def hello():\n    print(\"world\")\n    return True";
    let options = HighlightOptions {
      lang: Some("python".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<span class=\"line\">1</span>"));
    assert!(result.contains("<span class=\"line\">2</span>"));
    assert!(result.contains("<span class=\"line\">3</span>"));
    assert!(result.contains("hello"));
    assert!(result.contains("print"));
  }

  #[test]
  fn test_empty_code() {
    let code = "";
    let options = HighlightOptions {
      lang: Some("javascript".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<figure class=\"highlight javascript\">"));
  }

  #[test]
  fn test_unknown_language_fallback() {
    let code = "test code";
    let options = HighlightOptions {
      lang: Some("unknown-lang-xyz".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("test code"));
  }

  #[test]
  fn test_syntax_highlighter_module() {
    let code = "const x = 42;";
    let result = syntax_highlighter::highlight_code_classed(code, "javascript");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<span class="));
  }

  #[test]
  fn test_syntax_highlighter_plaintext() {
    let code = "plain text";
    let result = syntax_highlighter::highlight_code_classed(code, "plaintext");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("plain text"));
  }

  #[test]
  fn test_nginx_language_mapping() {
    let code = "server {\n    listen 80;\n    server_name example.com;\n}";
    let options = HighlightOptions {
      lang: Some("nginx".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<figure class=\"highlight nginx\">"));
    assert!(result.contains("server"));
    assert!(result.contains("listen"));
    assert!(result.contains("<span class="));
  }

  #[test]
  fn test_typescript_language_mapping() {
    let code = "const x: number = 42;";
    let options = HighlightOptions {
      lang: Some("typescript".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<span class="));
    assert!(result.contains("const"));
  }

  #[test]
  fn test_ts_extension_mapping() {
    let code = "type Foo = string;";
    let options = HighlightOptions {
      lang: Some("ts".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<span class="));
  }

  #[test]
  fn test_tsx_extension_mapping() {
    let code = "const Component = () => <div>Hello</div>;";
    let options = HighlightOptions {
      lang: Some("tsx".to_string()),
      ..Default::default()
    };

    let result = highlight(code, options);
    assert!(result.contains("<span class="));
  }

  #[test]
  fn test_normalize_language_function() {
    assert_eq!(normalize_language("nginx"), "perl");
    assert_eq!(normalize_language("typescript"), "javascript");
    assert_eq!(normalize_language("ts"), "javascript");
    assert_eq!(normalize_language("tsx"), "javascript");
    assert_eq!(normalize_language("javascript"), "javascript");
    assert_eq!(normalize_language("python"), "python");
  }
}
