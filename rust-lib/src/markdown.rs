use markdown_it;
use markdown_it_footnote;

// WIP
pub fn render(input: &str) -> String {
  let parser = &mut markdown_it::MarkdownIt::new();
  markdown_it::plugins::cmark::add(parser);
  markdown_it::plugins::extra::add(parser);
  markdown_it_footnote::add(parser);

  let ast = parser.parse(input);
  ast.render()
}
