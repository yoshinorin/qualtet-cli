const codeBlockFormatter = require("./codeBlockFormatter");
const { removeTemplateEnginesSyntax } = require("../rust-lib/index.js");
const { generateRobots } = require("../lib/robots.js");
const { externalLink } = require("./externalLink");
const micromatch = require("micromatch");
const { renderMarkdown } = require("../lib/markdown.js");
const log = require("hexo-log").default({
  debug: false,
  silent: false,
});

function objectsGenerator(content, contentType, url) {
  let tags = [];
  if (content.tags && content.tags != undefined) {
    content.tags.forEach((t) => {
      tags.push(t.name);
    });
  }

  let external_resources = [];
  if (content.external_resources) {
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
  }

  let path = content.path.endsWith("/")
    ? content.path
    : content.path.slice(0, content.path.lastIndexOf("/") + 1);
  if (!path.startsWith("/")) {
    path = "/" + path;
  }
  if (contentType === "article" && !path.includes("/articles")) {
    path = "/articles" + path;
  }
  const c = removeTemplateEnginesSyntax(content._content);

  if (
    micromatch.isMatch(path, [
      "temp/*",
      "all-categories/",
      "all-archives/",
      "scaffolds/",
      "404/",
      "_drafts/",
    ])
  ) {
    log.info(`skiped - : ${path}`);
    return;
  }

  const htmlContent = externalLink(
    renderMarkdown(codeBlockFormatter.format(c)),
    url,
  );
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

  if (tags.length != 0) {
    Object.assign(data, { tags: tags });
  }

  if (external_resources.length != 0) {
    Object.assign(data, { externalResources: external_resources });
  }

  if (content.series && content.series.trim().length != 0) {
    Object.assign(data, { series: content.series });
  }
  return data;
}

module.exports = {
  objectsGenerator,
};
