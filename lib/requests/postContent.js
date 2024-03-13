function postContent(httpClientWithAuth, data) {
  return httpClientWithAuth.post(
    'v1/contents',
    data
  )
}

module.exports = {
  postContent
};