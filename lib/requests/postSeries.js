function postSeries(httpClientWithAuth, data) {
  return httpClientWithAuth.post(
    'series',
    data
  )
}

module.exports = {
  postSeries
};