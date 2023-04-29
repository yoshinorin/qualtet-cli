const log = require('hexo-log').default({
  debug: false,
  silent: false
});

const API_URL = process.argv[2];
const service = process.argv[3];
const authorName = process.argv[4];
const tagId = process.argv[5];

const axios = require("axios");
const { deleteTagRequest } = require('../lib/requests/deleteTag');
const { invalidateCache } = require('../lib/requests/invalidateCaches');
const { getAuthorId, getJwt } = require('../lib/auth');
const { getCredential } = require('../lib/getCredential');

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
    deleteTagRequest(httpClientWithAuth, tagId)
    invalidateCache(httpClientWithAuth);
    log.info(`caches: invalidated`);
  } catch(err) {
    log.error(err);
  } finally {
    // Nothing todo
  }
})();
