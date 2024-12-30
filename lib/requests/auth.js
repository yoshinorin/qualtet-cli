const { logError } = require("../../rust-lib/index.js");
const { exit } = require("process");

async function getAuthorId(httpClient, author) {
  return httpClient
    .get(`v1/authors/${author}`)
    .then((response) => {
      return response.data;
    })
    .catch((error) => {
      logError(error.response.status.toString());
      logError(error.response.statusText);
      logError(error.response.headers);
      logError(error.response.data);
      exit(0);
    });
}

async function getJwt(httpClient, authorId, password) {
  const a = await authorId;
  const p = await password;

  return httpClient
    .post(
      "v1/token",
      `{
    "authorId": "${a.id}",
    "password": "${p}"
  }`,
    )
    .then((response) => {
      console.log(response.data);
      return response.data.token;
    })
    .catch((error) => {
      logError(error.response.status.toString());
      logError(error.response.statusText);
      logError(error.response.headers.toString());
      logError(error.response.data.toString());
      exit(1);
    });
}

module.exports = {
  getAuthorId,
  getJwt,
};
