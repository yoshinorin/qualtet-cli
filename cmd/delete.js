const { logInfo, logError } = require("../rust-lib/index.js");
const { deleteContent } = require("../lib/requests/deleteContent");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { parseCommonArgs } = require("../lib/parseCommonArgs");

const {
  apiUrl,
  service,
  authorName,
  "content-id": contentId,
} = parseCommonArgs({
  "content-id": { type: "string" },
});

(async () => {
  const token = await getAuthToken(apiUrl, service, authorName);

  try {
    deleteContent(apiUrl, token, contentId);
    invalidateCache(apiUrl, token);
    logInfo(`Caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
