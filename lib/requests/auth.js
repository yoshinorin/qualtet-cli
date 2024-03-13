const log = require('hexo-log').default({
  debug: false,
  silent: false
});
const { exit } = require("process");

async function getAuthorId(httpClient, author) {
  return httpClient.get(`v1/authors/${author}`)
    .then(response => {
      return response.data;
    }).catch(error => {
      log.error(error.response.status);
      log.error(error.response.statusText);
      log.error(error.response.headers);
      log.error(error.response.data);
      exit(0);
    })
}

async function getJwt(httpClient, authorId, password) {

  const a = await authorId
  const p = await password

  return httpClient.post('v1/token', `{
    "authorId": "${a.id}",
    "password": "${p}"
  }`)
  .then(response => {
    console.log(response.data);
    return response.data.token;
  })
  .catch(error => {
    log.error(error.response.status);
    log.error(error.response.statusText);
    log.error(error.response.headers);
    log.error(error.response.data);
    exit(1);
  });
}

module.exports = {
  getAuthorId,
  getJwt
}
