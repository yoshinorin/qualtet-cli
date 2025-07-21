const codeBlockFormatter = require("../contents/codeBlockFormatter.js");
const {
  removeTemplateEnginesSyntax,
  generateRobots,
  formatPath,
} = require("../../rust-lib/index.js");
const { externalLink } = require("./externalLinkReplacer.js");
const { renderMarkdown } = require("./markdownRenderer.js");

function generatePayload(content, contentType, url) {
  const path = formatPath(content.path, contentType);
  const c = removeTemplateEnginesSyntax(content._content);

  const formattedCodeBlockMarkdown = codeBlockFormatter.format(c);
  const renderedMarkdown = renderMarkdown(formattedCodeBlockMarkdown);
  const htmlContent = externalLink(renderedMarkdown, url);
  const data = {
    contentType: contentType,
    path: path,
    title: content.title,
    robotsAttributes: generateRobots(content.noindex, contentType),
    rawContent: c,
    htmlContent: htmlContent,
    publishedAt: content.date.unix(),
    updatedAt: content.updated.unix(),
  };

  if (content.tags && content.tags != undefined) {
    let tags = [];
    content.tags.forEach((t) => {
      tags.push({
        name: t.name,
        path: t.name.replaceAll(" ", "-").replaceAll("'", "")
      });
    });
    if (tags.length != 0) {
      Object.assign(data, { tags: tags });
    }
  }

  if (content.external_resources) {
    let external_resources = [];
    let js = [];
    if (content.external_resources.js) {
      content.external_resources.js.forEach((j) => {
        js.push(j);
      });
      external_resources.push({ kind: "js", values: js });
    }

    let css = [];
    if (content.external_resources.css) {
      content.external_resources.css.forEach((c) => {
        css.push(c);
      });
      external_resources.push({ kind: "css", values: css });
    }
    if (external_resources.length != 0) {
      Object.assign(data, { externalResources: external_resources });
    }
  }

  if (content.series && content.series.trim().length != 0) {
    Object.assign(data, { series: content.series });
  }
  return data;
}

module.exports = {
  generatePayload,
};
