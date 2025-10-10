const { logInfo, logError } = require("../rust-lib/index.js");
const { deleteTagRequest } = require("../lib/requests/deleteTag");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { parseCommonArgs } = require("../lib/parseCommonArgs");

const {
  apiUrl,
  service,
  authorName,
  "tag-id": tagId,
} = parseCommonArgs({
  "tag-id": { type: "string" },
});

(async () => {
  const token = await getAuthToken(apiUrl, service, authorName);

  try {
    deleteTagRequest(apiUrl, token, tagId);
    invalidateCache(apiUrl, token);
    logInfo(`caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
