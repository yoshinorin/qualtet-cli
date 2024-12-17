function deleteTagRequest(httpClientWithAuth, id) {
  return httpClientWithAuth.delete(`v1/tags/${id}`);
}

module.exports = {
  deleteTagRequest,
};
