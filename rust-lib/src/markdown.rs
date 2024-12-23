use markdown_it;

// WIP
pub fn render(input: &str) -> String {
  let parser = &mut markdown_it::MarkdownIt::new();
  let ast = parser.parse(input);
  ast.render()
}
