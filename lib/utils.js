/**
 * @deprecated
 */
export function removeTemplateEnginesSyntax(text) {
  return text.replaceAll("{% raw %}", "").replaceAll("{% endraw %}", "");
}

export function formatPath(path, contentType) {
  let p = path.endsWith("/") ? path : path + "/";

  if (!p.startsWith("/")) {
    p = "/" + p;
  }
  if (contentType === "article" && !p.includes("/articles")) {
    p = "/articles" + p;
  }
  return p;
}
