use markdown_it_footnote;
use markdown_it_lazyload;

// WIP
pub fn render(input: &str) -> String {
  let parser = &mut markdown_it::MarkdownIt::new();
  markdown_it::plugins::cmark::add(parser);
  markdown_it::plugins::extra::add(parser);
  markdown_it_footnote::add(parser);
  markdown_it_lazyload::add(parser);

  let ast = parser.parse(input);
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
