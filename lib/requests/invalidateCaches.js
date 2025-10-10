const { httpDelete } = require("../../rust-lib/index.js");

function invalidateCache(apiUrl, token) {
  return httpDelete(apiUrl, "v1/caches", token);
}

module.exports = {
  invalidateCache,
};
