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
  "reload-url": reloadUrl,
} = parseCommonArgs({
  // Directory path for storing assets to be deployed (e.g., via rsync).
  // Actual deployment is not handled by this CLI - implement it separately using shell scripts, etc.
  "deploy-assets-dir": { type: "string" },
  "reload-url": { type: "string" },
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

  logInfo(`File changed: ${file.path}`);

  // NOTE: Never let this function reject. Hexo runs processors sequentially via
  // `Bluebird.reduce`, so a rejection aborts the chain and Hexo's built-in
  // processors never refresh the in-memory DB that `findByPath` reads, freezing
  // the content until the process is restarted.
  try {
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
    if (reloadUrl) {
      fetch(reloadUrl)
        .then(async (res) => {
          const body = await res.text();
          logInfo(`GET ${reloadUrl} - ${res.status}: ${body}`);
        })
        .catch((err) => {
          logError(
            `Failed to send GET request to ${reloadUrl}: ${err.message}`,
          );
        });
    }
    if (data) {
      copyContentAssets(item, {
        contentType,
        deployAssetsDir,
        hexo,
      });
    }
  } catch (err) {
    logError(`Failed to handle file update: ${file.path} - ${err}`);
  }
}

(async () => {
  await waitForApiServerReady(apiUrl, 120);
  token = await getAuthToken(apiUrl, service, authorName);

  try {
    invalidateCache(apiUrl, token);
    logInfo(`Caches: invalidated`);
  } catch (err) {
    logError(err.toString());
  }

  await hexo.init();

  // NOTE: Register the processor AFTER hexo.init(). Hexo executes processors in
  // registration order, and its built-in `post`/`page` processors - which update
  // the in-memory DB that `findByPath` reads - are registered during init().
  // Registering earlier makes this processor run first and publish the content
  // of the previous save.
  hexo.extend.processor.register("**/*.md", handleFileUpdate);
  logInfo("Hexo initialized. Watching for file changes...");

  await hexo.watch();
  watching = true;
  logInfo("Watch mode started. Files will be sent to API on change.");
})();
