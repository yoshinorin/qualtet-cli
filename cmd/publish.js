const Hexo = require("hexo");
const hexo = new Hexo(process.cwd(), { silent: false });
const fs = require("fs-extra");
const { join } = require("path");
const log = require("hexo-log").default({
  debug: false,
  silent: false,
});

const {
  httpClientWithNonAuth,
  httpClientWithAuth,
} = require("../lib/httpClients");
const { objectsGenerator } = require("../lib/objectsGenerator");
const { postContent } = require("../lib/requests/postContent");
const { invalidateCache } = require("../lib/requests/invalidateCaches");
const { getAuthorId, getJwt } = require("../lib/requests/auth");
const { getCredential } = require("../lib/getCredential.js");
const { validate } = require("../lib/imageValidation");
const { shouldSkipPaths } = require("../rust-lib/index.js");

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
  const generatedContent = objectsGenerator(content, type, baseUrl);
  if (!generatedContent) {
    return null;
  }
  return generatedContent;
}

function responseErrorHandler(content, error) {
  try {
    log.error(error.response.status);
    log.error(`error: - ${content.path}`);
  } catch {
    // Nothing todo
  }
}

(async () => {
  const password = getCredential(service, authorName);
  const author = getAuthorId(httpClientWithNonAuth(API_URL), authorName);
  const token = await getJwt(httpClientWithNonAuth(API_URL), author, password);

  try {
    invalidateCache(httpClientWithAuth(API_URL, token));
    log.info(`caches: invalidated`);
  } catch (err) {
    log.error(err);
  } finally {
    // Nothing todo
  }

  const daysAgo = process.argv[5] ? process.argv[5] : 5;

  let cnt = 0;
  hexo.init().then(() => {
    hexo.load().then(() => {
      let date = new Date();
      date = date.setDate(date.getDate() - daysAgo);
      const wait = (ms) => new Promise((r) => setTimeout(r, ms));
      const postAsset = hexo.model("PostAsset");
      const pageAsset = hexo.model("Asset");
      const url = hexo.config.url;

      (async () => {
        const posts = hexo.locals.get("posts").filter((c) => c.updated > date);
        for (let post of posts.toArray()) {
          const content = generatePostContent(post, "article", url);
          if (content == null) {
            continue;
          }
          postContent(httpClientWithAuth(API_URL, token), content)
            .then((response) => {
              cnt++;
              log.info(
                `created - ${cnt}: ${response.data.id} - ${response.data.path}`,
              );
              const assets = postAsset.find({ post: post._id }).toArray();
              assets.forEach(async (a) => {
                if (await validate(a.source)) {
                  fs.copy(
                    a.source,
                    join(
                      hexo.base_dir,
                      "_staticContentAssets",
                      "articles",
                      a.path,
                    ),
                    (err) => {
                      if (err) {
                        log.error(err);
                      }
                    },
                  );
                } else {
                  log.error(`image copy skipped - : ${content.path}`);
                }
              });
            })
            .catch((error) => {
              responseErrorHandler(post, error);
            });
          await wait(150);
        }
      })();

      // TODO: DRY
      (async () => {
        // TODO: excludes scaffolds
        const pages = hexo.locals.get("pages").filter((c) => c.updated > date);
        for (let page of pages.toArray()) {
          const content = generatePostContent(page, "page", url);
          if (content == null) {
            continue;
          }
          postContent(httpClientWithAuth(API_URL, token), content)
            .then((response) => {
              cnt++;
              log.info(
                `created - ${cnt}: ${response.data.id} - ${response.data.path}`,
              );
              const pageDir = page.path.slice(0, page.path.lastIndexOf("/"));
              const assets = pageAsset.filter((x) => x._id.includes(pageDir));
              assets.forEach(async (a) => {
                if (await validate(a.source)) {
                  fs.copy(
                    a.source,
                    join(hexo.base_dir, "_staticContentAssets", a.path),
                    (err) => {
                      if (err) {
                        log.error(err);
                      }
                    },
                  );
                } else {
                  log.error(`image copy skipped - : ${content.path}`);
                }
              });
            })
            .catch((error) => {
              responseErrorHandler(page, error);
            });
          await wait(150);
        }
      })();
    });
  });
})();
