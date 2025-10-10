const { httpPost } = require("../../rust-lib/index.js");

function postContent(apiUrl, token, data) {
  const jsonData = typeof data === "string" ? data : JSON.stringify(data);
  return httpPost(apiUrl, "v1/contents", jsonData, token);
}

module.exports = {
  postContent,
};
