const { logInfo, logError } = require("../rust-lib/index.js");
const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];
const tagId = process.argv[5];

const { deleteTagRequest } = require("../lib/requests/deleteTag");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");

(async () => {
  const token = await getAuthToken(API_URL, service, authorName);

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
