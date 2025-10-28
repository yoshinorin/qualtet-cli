const { logInfo, logError } = require("../rust-lib/index.js");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { parseCommonArgs } = require("../lib/parseCommonArgs");

const { apiUrl, service, authorName } = parseCommonArgs();

(async () => {
  const token = await getAuthToken(apiUrl, service, authorName);

  try {
    invalidateCache(apiUrl, token);
    logInfo(`Caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
