const { shouldSkipPaths } = require("../../rust-lib/index.js");
const { generatePayload } = require("./generator.js");
const { SKIP_PATHS } = require("../constants.js");

function generatePostContent(content, type, baseUrl) {
  if (shouldSkipPaths(content.path, SKIP_PATHS)) {
    return null;
  }
  const generatedContent = generatePayload(content, type, baseUrl);
  if (!generatedContent) {
    return null;
  }
  return generatedContent;
}

module.exports = {
  generatePostContent,
};
