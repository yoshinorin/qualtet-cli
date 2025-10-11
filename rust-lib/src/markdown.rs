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

pub fn render(input: &str) -> String {
  let ast = PARSER.parse(input);
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
}
