const {
  logInfo,
  logError,
  shouldSkipPaths,
} = require("../../rust-lib/index.js");
const { generatePayload } = require("./generator.js");
const { postContent } = require("../requests/postContent.js");
const { SKIP_PATHS } = require("../constants.js");

async function publish(item, options) {
  const { contentType, apiUrl, token, baseUrl } = options;
  if (shouldSkipPaths(item.path, SKIP_PATHS)) {
    return null;
  }
  const payload = generatePayload(item, contentType, baseUrl);
  if (!payload) {
    return null;
  }

  try {
    const response = await postContent(apiUrl, token, payload);
    const data = JSON.parse(response);
    logInfo(`${contentType} published: ${data.id} - ${data.path}`);
    return data;
  } catch (error) {
    logError(`${contentType} publish failed: ${payload.path} - ${error}`);
    throw error;
  }
}

module.exports = {
  publish,
};
