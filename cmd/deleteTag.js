const log = require('hexo-log').default({
  debug: false,
  silent: false
});

const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];
const tagId = process.argv[5];

const { httpClientWithNonAuth, httpClientWithAuth } = require('../lib/httpClients');
const { deleteTagRequest } = require('../lib/requests/deleteTag');
const { invalidateCache } = require('../lib/requests/invalidateCaches');
const { getAuthorId, getJwt } = require('../lib/requests/auth');
const { getCredential } = require('../lib/getCredential');

(async () => {
  const password = getCredential(service, authorName)
  const author = getAuthorId(httpClientWithNonAuth(API_URL), authorName)
  const token = await getJwt(httpClientWithNonAuth(API_URL), author, password)

  try {
    deleteTagRequest(httpClientWithAuth(API_URL, token), tagId)
    invalidateCache(httpClientWithAuth(API_URL, token));
    log.info(`caches: invalidated`);
  } catch(err) {
    log.error(err);
  } finally {
    // Nothing todo
  }
})();
