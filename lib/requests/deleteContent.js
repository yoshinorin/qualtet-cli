const { httpDelete } = require("../../rust-lib/index.js");

function deleteContent(apiUrl, token, id) {
  return httpDelete(apiUrl, `v1/contents/${id}`, token);
}

module.exports = {
  deleteContent,
};
