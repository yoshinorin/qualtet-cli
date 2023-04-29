function invalidateCache(httpClientWithAuth) {
  return httpClientWithAuth.delete('caches')
}

module.exports = {
  invalidateCache
};