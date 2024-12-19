/**
 * @deprecated
 */
export function removeTemplateEnginesSyntax(text) {
  return text.replaceAll("{% raw %}", "").replaceAll("{% endraw %}", "");
}
