const { logInfo, logError } = require("../rust-lib/index.js");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { postSeries } = require("../lib/requests/postSeries");
const { parseCommonArgs } = require("../lib/parseCommonArgs");
const fs = require("fs");

const {
  apiUrl,
  service,
  authorName,
  "file-path": filePath,
} = parseCommonArgs({
  "file-path": { type: "string" },
});

(async () => {
  const token = await getAuthToken(apiUrl, service, authorName);

  try {
    const series = fs.readFileSync(filePath, "utf-8");
    JSON.parse(series).forEach((s) => {
      postSeries(apiUrl, token, s);
    });
    invalidateCache(apiUrl, token);
    logInfo(`Caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
