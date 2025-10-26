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
const { parseCommonArgs } = require("../lib/parseCommonArgs");

const {
  apiUrl,
  service,
  authorName,
  "days-ago": daysAgo = "5",
} = parseCommonArgs({
  "days-ago": { type: "string", default: "5" },
});

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
  const token = await getAuthToken(apiUrl, service, authorName);

  try {
    invalidateCache(apiUrl, token);
    logInfo(`caches: invalidated`);
  } catch (err) {
    logError(err);
  } finally {
    // Nothing todo
  }

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
        const response = await postContent(apiUrl, token, content);
        processedCount++;
        const data = JSON.parse(response);
        logInfo(`created - ${processedCount}: ${data.id} - ${data.path}`);

        const assets = assetFilterFn(item, assetModel);
        copyAssetsIfValid(assets, assetDestPath);
      } catch (error) {
        logError(`${content.path} - ${error}`);
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
