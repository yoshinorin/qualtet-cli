const { logInfo, logError } = require("../rust-lib/index.js");

const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];
const filePath = process.argv[5];

const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { postSeries } = require("../lib/requests/postSeries");
const fs = require("fs");

(async () => {
  const token = await getAuthToken(API_URL, service, authorName);

  try {
    const series = fs.readFileSync(filePath, "utf-8");
    JSON.parse(series).forEach((s) => {
      postSeries(API_URL, token, s);
    });
    invalidateCache(API_URL, token);
    logInfo(`caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }
})();
