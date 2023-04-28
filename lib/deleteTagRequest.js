function deleteTagRequest(httpClientWithAuth, id) {
  return httpClientWithAuth.delete(`tags/${id}`)
}

module.exports = {
  deleteTagRequest
};