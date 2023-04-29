function deleteContent(httpClientWithAuth, id) {
  return httpClientWithAuth.delete(`contents/${id}`)
}

module.exports = {
  deleteContent
};