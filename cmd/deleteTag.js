const { logInfo, logError } = require("../rust-lib/index.js");
const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];
const tagId = process.argv[5];

const { deleteTagRequest } = require("../lib/requests/deleteTag");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthorId, getJwt } = require("../lib/requests/auth");
const { getCredential } = require("../lib/getCredential.js");

(async () => {
  const password = getCredential(service, authorName);
  const author = getAuthorId(API_URL, authorName);
  const token = await getJwt(API_URL, author, password);

  try {
    deleteTagRequest(API_URL, token, tagId);
    invalidateCache(API_URL, token);
    logInfo(`caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
