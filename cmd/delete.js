const { logInfo, logError } = require("../rust-lib/index.js");

const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];
const contentId = process.argv[5];

const { deleteContent } = require("../lib/requests/deleteContent");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");

(async () => {
  const token = await getAuthToken(API_URL, service, authorName);

  try {
    deleteContent(API_URL, token, contentId);
    invalidateCache(API_URL, token);
    logInfo(`caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
