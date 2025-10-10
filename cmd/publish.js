const Hexo = require("hexo");
const hexo = new Hexo(process.cwd(), { silent: false });
const fs = require("fs-extra");
const { join } = require("path");
const { logInfo, logError } = require("../rust-lib/index.js");

const { generatePayload } = require("../lib/contents/generator");
const { postContent } = require("../lib/requests/postContent");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthToken } = require("../lib/requests/auth");
const { shouldSkipPaths, isValidImage } = require("../rust-lib/index.js");

const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];

const SKIP_PATHS = [
  "temp/**",
  "all-categories/**",
  "all-archives/**",
  "scaffolds/**",
  "404/**",
  "_drafts/**",
];

function generatePostContent(content, type, baseUrl) {
  if (shouldSkipPaths(content.path, SKIP_PATHS)) {
    return null;
  }
  const generatedContent = generatePayload(content, type, baseUrl);
  if (!generatedContent) {
    return null;
  }
  return generatedContent;
}

function responseErrorHandler(content, error) {
  try {
    logError(error.response.status.toString());
    logError(`error: - ${content.path}`);
  } catch {
    // Nothing todo
  }
}

function copyAssetsIfValid(assets, dest) {
  assets.forEach((a) => {
    if (isValidImage(a.source)) {
      fs.copy(a.source, join(dest, a.path), (err) => {
        if (err) {
          logError(err);
        }
      });
    } else {
      logError(`image copy skipped - : ${a.path}`);
    }
  });
}

(async () => {
  const token = await getAuthToken(API_URL, service, authorName);

  try {
    invalidateCache(API_URL, token);
    logInfo(`caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }

  const daysAgo = process.argv[5] ? process.argv[5] : 5;

  async function processContents(
    contents,
    contentType,
    url,
    assetModel,
    assetDestPath,
    assetFilterFn,
  ) {
    const wait = (ms) => new Promise((r) => setTimeout(r, ms));
    let processedCount = 0;

    for (let item of contents.toArray()) {
      const content = generatePostContent(item, contentType, url);
      if (content == null) {
        continue;
      }

      try {
        const response = await postContent(API_URL, token, content);
        processedCount++;
        const data = JSON.parse(response);
        logInfo(`created - ${processedCount}: ${data.id} - ${data.path}`);

        const assets = assetFilterFn(item, assetModel);
        copyAssetsIfValid(assets, assetDestPath);
      } catch (error) {
        responseErrorHandler(item, error);
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
      const postAsset = hexo.model("PostAsset");
      const pageAsset = hexo.model("Asset");
      const url = hexo.config.url;

      const posts = hexo.locals.get("posts").filter((c) => c.updated > date);
      cnt += await processContents(
        posts,
        "article",
        url,
        postAsset,
        join(hexo.base_dir, "_staticContentAssets", "articles"),
        (post, assetModel) => assetModel.find({ post: post._id }).toArray(),
      );

      const pages = hexo.locals.get("pages").filter((c) => c.updated > date);
      cnt += await processContents(
        pages,
        "page",
        url,
        pageAsset,
        join(hexo.base_dir, "_staticContentAssets"),
        (page, assetModel) => {
          const pageDir = page.path.slice(0, page.path.lastIndexOf("/"));
          return assetModel.filter((x) => x._id.includes(pageDir));
        },
      );
    });
  });
})();
