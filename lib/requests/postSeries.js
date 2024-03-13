function postSeries(httpClientWithAuth, data) {
  return httpClientWithAuth.post(
    'v1/series',
    data
  )
}

module.exports = {
  postSeries
};