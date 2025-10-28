const { logInfo, logError } = require("../../rust-lib/index.js");
const { generatePostContent } = require("./utils.js");
const { postContent } = require("../requests/postContent.js");

async function publish(item, options) {
  const { contentType, apiUrl, token, baseUrl } = options;

  const content = generatePostContent(item, contentType, baseUrl);
  if (!content) {
    return null;
  }

  try {
    const response = await postContent(apiUrl, token, content);
    const data = JSON.parse(response);
    logInfo(`${contentType} published: ${data.id} - ${data.path}`);
    return data;
  } catch (error) {
    logError(`${contentType} publish failed: ${content.path} - ${error}`);
    throw error;
  }
}

module.exports = {
  publish,
};
