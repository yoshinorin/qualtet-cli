use markdown_it::generics::inline::full_link;
use markdown_it::{self, NodeValue};
use markdown_it_footnote;

// WIP
pub fn render(input: &str) -> String {
  let parser = &mut markdown_it::MarkdownIt::new();
  markdown_it::plugins::cmark::add(parser);
  markdown_it::plugins::extra::add(parser);
  markdown_it_footnote::add(parser);
  add_custom_image(parser);

  let ast = parser.parse(input);
  ast.render()
}

#[derive(Debug)]
struct CustomImage {
  pub url: String,
  pub title: Option<String>,
}

impl NodeValue for CustomImage {
  fn render(&self, node: &markdown_it::Node, fmt: &mut dyn markdown_it::Renderer) {
    let mut attrs = node.attrs.clone();

    attrs.push(("src", self.url.clone()));
    attrs.push(("alt", node.collect_text()));
    attrs.push(("loading", "lazy".to_string()));

    if let Some(title) = &self.title {
      attrs.push(("title", title.clone()));
    }

    fmt.self_close("img", &attrs);
  }
}

fn add_custom_image(md: &mut markdown_it::MarkdownIt) {
  full_link::add_prefix::<'!', true>(md, |href, title| {
    markdown_it::Node::new(CustomImage {
      url: href.unwrap_or_default(),
      title,
    })
  });
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
