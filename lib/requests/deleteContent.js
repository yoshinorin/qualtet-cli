function deleteContent(httpClientWithAuth, id) {
  return httpClientWithAuth.delete(`v1/contents/${id}`)
}

module.exports = {
  deleteContent
};