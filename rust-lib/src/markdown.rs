use std::sync::LazyLock;

use markdown_it::MarkdownIt;
use markdown_it_footnote;
use markdown_it_lazyload;

static PARSER: LazyLock<MarkdownIt> = LazyLock::new(|| {
  let mut parser = markdown_it::MarkdownIt::new();

  markdown_it::plugins::cmark::add(&mut parser);

  markdown_it::plugins::extra::smartquotes::add(&mut parser);
  markdown_it::plugins::extra::tables::add(&mut parser);
  markdown_it::plugins::extra::strikethrough::add(&mut parser);
  markdown_it::plugins::extra::syntect::add(&mut parser);
  markdown_it::plugins::extra::typographer::add(&mut parser);
  markdown_it::plugins::html::add(&mut parser);

  markdown_it_footnote::definitions::add(&mut parser);
  markdown_it_footnote::references::add(&mut parser);
  markdown_it_footnote::inline::add(&mut parser);
  markdown_it_footnote::collect::add(&mut parser);
  markdown_it_footnote::back_refs::add(&mut parser);

  markdown_it_lazyload::add(&mut parser);

  parser
});

/// Preprocesses blockquotes to render consecutive blockquote lines as separate paragraphs.
///
/// This function inserts an empty blockquote line (">") between consecutive blockquote lines,
/// causing the markdown parser to render each line as a separate paragraph within the same blockquote.
///
/// # Examples
///
/// ```
/// // Input:
/// // > Line 1
/// // > Line 2
///
/// // After preprocessing:
/// // > Line 1
/// // >
/// // > Line 2
///
/// // Rendered output:
/// // <blockquote>
/// // <p>Line 1</p>
/// // <p>Line 2</p>
/// // </blockquote>
/// ```
fn preprocess_blockquotes(input: &str) -> String {
  let lines: Vec<&str> = input.lines().collect();
  let mut result = Vec::new();
  let mut prev_was_blockquote = false;

  for line in lines {
    let trimmed = line.trim_start();
    let is_blockquote = trimmed.starts_with('>');
    let is_empty_blockquote = trimmed == ">";

    if is_blockquote && prev_was_blockquote && !is_empty_blockquote {
      result.push(">");
    }

    result.push(line);
    prev_was_blockquote = is_blockquote;
  }

  result.join("\n")
}

pub fn render(input: &str) -> String {
  let preprocessed = preprocess_blockquotes(input);
  let ast = PARSER.parse(&preprocessed);
  ast.render()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render() {
    let input = "![Rust](https://example.com/example.png)";
    let output = render(input);

    assert_eq!(
      output,
      "<p><img src=\"https://example.com/example.png\" alt=\"Rust\" loading=\"lazy\"></p>\n"
    );
  }

  #[test]
  fn test_consecutive_blockquote_lines() {
    let input = "> Line 1\n> Line 2\n> Line 3";
    let output = render(input);

    assert_eq!(
      output,
      "<blockquote>\n<p>Line 1</p>\n<p>Line 2</p>\n<p>Line 3</p>\n</blockquote>\n"
    );
  }

  #[test]
  fn test_blockquote_with_existing_blank_lines() {
    let input = "> Line 1\n>\n> Line 2";
    let output = render(input);

    assert_eq!(
      output,
      "<blockquote>\n<p>Line 1</p>\n<p>Line 2</p>\n</blockquote>\n"
    );
  }

  #[test]
  fn test_single_blockquote_line() {
    let input = "> Single line";
    let output = render(input);

    assert_eq!(output, "<blockquote>\n<p>Single line</p>\n</blockquote>\n");
  }
}
