function invalidateCache(httpClientWithAuth) {
  return httpClientWithAuth.delete('v1/caches')
}

module.exports = {
  invalidateCache
};