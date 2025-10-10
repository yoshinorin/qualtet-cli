const { httpPost } = require("../../rust-lib/index.js");

function postSeries(apiUrl, token, data) {
  const jsonData = typeof data === "string" ? data : JSON.stringify(data);
  return httpPost(apiUrl, "v1/series", jsonData, token);
}

module.exports = {
  postSeries,
};
