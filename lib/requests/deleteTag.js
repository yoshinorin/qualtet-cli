const { httpDelete } = require("../../rust-lib/index.js");

function deleteTagRequest(apiUrl, token, id) {
  return httpDelete(apiUrl, `v1/tags/${id}`, token);
}

module.exports = {
  deleteTagRequest,
};
