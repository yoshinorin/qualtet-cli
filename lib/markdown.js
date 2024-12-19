import MarkdownIt from "markdown-it";
import markdownItFootnote from "markdown-it-footnote";

const md = new MarkdownIt({
  html: true,
  xhtmlOut: false,
  breaks: true,
  linkify: true,
  typographer: true,
  quotes: "“”‘’",
}).use(markdownItFootnote);

// lazy load
md.renderer.rules.image = function (tokens, idx, options, env, self) {
  const token = tokens[idx];
  token.attrSet("loading", "lazy");
  return self.renderToken(tokens, idx, options);
};

export function renderMarkdown(text) {
  return md.render(text);
}
