const Hexo = require('hexo');
const hexo = new Hexo(process.cwd(), {silent: false});
const fs = require('fs-extra');
const { join } = require('path');
const log = require('hexo-log').default({
  debug: false,
  silent: false
});
const axios = require("axios");
const { objectsGenerator } = require('../lib/objectsGenerator');
const { postContent } = require('../lib/requests/postContent');
const { invalidateCache } = require('../lib/requests/invalidateCaches');
const { getCredential, getAuthorId, getJwt } = require('../lib/auth');
const { validate } = require('../lib/imageValidation');

const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];

const httpClientWithNonAuth = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  }
});

(async () => {
  const password = getCredential(service, authorName)
  const author = getAuthorId(httpClientWithNonAuth, authorName)
  const token = await getJwt(httpClientWithNonAuth, author, password)

  log.info(token);

  const httpClientWithAuth = axios.create({
    baseURL: API_URL,
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    }
  });

  try {
    invalidateCache(httpClientWithAuth);
    log.info(`caches: invalidated`);
  } catch(err) {
    log.error(err);
  } finally {
    // Nothing todo
  }

  const daysAgo = process.argv[5] ? process.argv[5] : 5;

  let cnt = 0;
  let errorCnt = 0;
  hexo.init().then(() => {
    hexo.load().then(() => {

      let date = new Date();
      date = date.setDate(date.getDate() - daysAgo);
      const wait = (ms) => new Promise(r => setTimeout(r, ms));
      const postAsset = hexo.model('PostAsset');
      const pageAsset = hexo.model('Asset');
      const url = hexo.config.url;

      (async () => {
        const posts = hexo.locals.get('posts').filter(c => c.updated > date);
        for (let post of posts.toArray()) {
          const p = objectsGenerator(post, 'article', url);
          if (!p) {
            continue;
          }
          postContent(httpClientWithAuth, p).then(response => {
            cnt++;
            log.info(`created - ${cnt}: ${response.data.id} - ${response.data.path}`);
            const assets = postAsset.find({post: post._id}).toArray();
            assets.forEach(a => {
              validate(a.source)
              // TODO: validate image
              //if (validate(a.source)) {
                fs.copy(a.source, join(hexo.base_dir, '_staticContentAssets', 'articles', a.path), (err) => {
                  if (err) {
                    log.error(err)
                  }
                });
              //}
            });
          })
          .catch(error => {
            try {
              log.error(error.response.status);
              errorCnt++
              log.error(`error: - ${errorCnt} ${post.path}`);
            } catch {
              // Nothing todo
            }
          });
          await wait(150);
        }
      })();

      // TODO: DRY
      (async () => {
        // TODO: excludes scaffolds
        const pages = hexo.locals.get('pages').filter(c => c.updated > date)
        for(let page of pages.toArray()) {
          const p = objectsGenerator(page, 'page', url);
          if (!p) {
            continue;
          }
          postContent(httpClientWithAuth, p).then(response => {
            cnt++;
            log.info(`created - ${cnt}: ${response.data.id} - ${response.data.path}`);
            const pageDir = page.path.slice(0, page.path.lastIndexOf("/"));
            const assets = pageAsset.filter(x => x._id.includes(pageDir));
            assets.forEach(a => {
              // TODO: validate image
              // if (validate(a.source)) {
                fs.copy(a.source, join(hexo.base_dir, '_staticContentAssets', a.path), (err) => {
                  if (err) {
                    log.error(err)
                  }
                });
              // }
            });
          })
          .catch(error => {
            try {
              log.error(error.response.status);
              errorCnt++
              log.error(`error: - ${errorCnt} ${post.path}`);
            } catch {
              // Nothing todo
            }
          });
          await wait(150);
        }
      })();
    });
  });
})();
