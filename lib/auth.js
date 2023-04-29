const { exit } = require("process");

async function getAuthorId(httpClient, author) {
  return httpClient.get(`authors/${author}`)
    .then(response => {
      return response.data;
    }).catch(error => {
      console.log(error.response.status);
      console.log(error.response.statusText);
      console.log(error.response.headers);
      console.log(error.response.data);
      exit(0);
    })
}

async function getJwt(httpClient, authorId, password) {

  const a = await authorId
  const p = await password

  return httpClient.post('token', `{
    "authorId": "${a.id}",
    "password": "${p}"
  }`)
  .then(response => {
    console.log(response.data);
    return response.data.token
  })
  .catch(error => {
    console.log(error.response.status);
    console.log(error.response.statusText);
    console.log(error.response.headers);
    console.log(error.response.data);
    exit(1);
  });
}

module.exports = {
  getAuthorId,
  getJwt
}
