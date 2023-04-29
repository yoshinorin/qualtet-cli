function postContent(httpClientWithAuth, data) {
  return httpClientWithAuth.post(
    'contents',
    data
  )
}

module.exports = {
  postContent
};