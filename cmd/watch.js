const Hexo = require("hexo");
const hexo = new Hexo(process.cwd(), { silent: false });
const { logInfo, logError } = require("../rust-lib/index.js");

const { findByPath } = require("../lib/contents/hexoContentResolver.js");
const { publish } = require("../lib/contents/publisher.js");
const { copyContentAssets } = require("../lib/contents/assets.js");
const { invalidateCache } = require("../lib/requests/invalidateCaches.js");
const { getAuthToken } = require("../lib/requests/auth.js");
const { waitForApiServerReady } = require("../lib/requests/healthCheck.js");
const { parseCommonArgs } = require("../lib/parseCommonArgs.js");

const {
  apiUrl,
  service,
  authorName,
  "deploy-assets-dir": deployAssetsDir,
} = parseCommonArgs({
  // Directory path for storing assets to be deployed (e.g., via rsync).
  // Actual deployment is not handled by this CLI - implement it separately using shell scripts, etc.
  "deploy-assets-dir": { type: "string" },
});

// Validate required argument: deploy-assets-dir
if (!deployAssetsDir) {
  logError("Error: --deploy-assets-dir is required");
  process.exit(1);
}

let token = null;
let watching = false;

async function handleFileUpdate(file) {
  if (!watching) {
    return;
  }

  if (file.type !== "update" && file.type !== "create") {
    return;
  }

  // NOTE: Wait for Hexo to update its internal content cache
  await new Promise((resolve) => setTimeout(resolve, 500));
  logInfo(`File changed: ${file.path}`);

  const result = findByPath(hexo, file.path);
  if (!result) {
    logInfo(`Content not found for: ${file.path}`);
    return;
  }

  const { item, contentType } = result;
  const data = await publish(item, {
    contentType,
    apiUrl,
    token,
    baseUrl: hexo.config.url,
  });
  if (data) {
    copyContentAssets(item, {
      contentType,
      deployAssetsDir,
      hexo,
    });
  }
}

// Register processor BEFORE hexo.init()
hexo.extend.processor.register("**/*.md", handleFileUpdate);

(async () => {
  await waitForApiServerReady(apiUrl, 120);
  token = await getAuthToken(apiUrl, service, authorName);

  try {
    invalidateCache(apiUrl, token);
    logInfo(`Caches: invalidated`);
  } catch (err) {
    logError(err);
  }

  hexo.init().then(() => {
    logInfo("Hexo initialized. Watching for file changes...");
    hexo.watch().then(() => {
      watching = true;
      logInfo("Watch mode started. Files will be sent to API on change.");
    });
  });
})();
