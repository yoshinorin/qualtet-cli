const Hexo = require("hexo");
const hexo = new Hexo(process.cwd(), { silent: false });
const { logInfo, logError } = require("../rust-lib/index.js");

const { publish } = require("../lib/contents/publisher.js");
const { copyContentAssets } = require("../lib/contents/assets.js");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { parseCommonArgs } = require("../lib/parseCommonArgs");

const {
  apiUrl,
  service,
  authorName,
  "days-ago": daysAgo = "5",
  "deploy-assets-dir": deployAssetsDir,
} = parseCommonArgs({
  "days-ago": { type: "string", default: "5" },
  // Directory path for storing assets to be deployed (e.g., via rsync).
  // Actual deployment is not handled by this CLI - implement it separately using shell scripts, etc.
  "deploy-assets-dir": { type: "string" },
});

// Validate required argument: deploy-assets-dir
if (!deployAssetsDir) {
  logError("Error: --deploy-assets-dir is required");
  process.exit(1);
}

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

  async function processContents(contents, contentType, url) {
    const wait = (ms) => new Promise((r) => setTimeout(r, ms));
    let processedCount = 0;

    for (let item of contents.toArray()) {
      try {
        const data = await publish(item, {
          contentType,
          apiUrl,
          token,
          baseUrl: url,
        });
        if (data) {
          copyContentAssets(item, {
            contentType,
            deployAssetsDir,
            hexo,
          });
        }
      } catch (error) {
        // Error already logged in publishContent
      } finally {
        processedCount++;
      }
      await wait(150);
    }
    return processedCount;
  }

  let cnt = 0;
  hexo.init().then(() => {
    hexo.load().then(async () => {
      let date = new Date();
      date = date.setDate(date.getDate() - daysAgo);
      const url = hexo.config.url;

      const posts = hexo.locals.get("posts").filter((c) => c.updated > date);
      cnt += await processContents(posts, "article", url);

      const pages = hexo.locals.get("pages").filter((c) => c.updated > date);
      cnt += await processContents(pages, "page", url);
    });
  });
})();
