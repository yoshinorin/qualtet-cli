const { logError, httpGet, httpPost } = require("../../rust-lib/index.js");
const { exit } = require("process");

async function getAuthorId(apiUrl, author) {
  try {
    const response = await httpGet(apiUrl, `v1/authors/${author}`);
    return JSON.parse(response);
  } catch (error) {
    logError(error.toString());
    exit(0);
  }
}

async function getJwt(apiUrl, authorId, password) {
  const a = await authorId;
  const p = await password;

  try {
    const response = await httpPost(
      apiUrl,
      "v1/token",
      `{
    "authorId": "${a.id}",
    "password": "${p}"
  }`,
    );
    const data = JSON.parse(response);
    console.log(data);
    return data.token;
  } catch (error) {
    logError(error.toString());
    exit(1);
  }
}

module.exports = {
  getAuthorId,
  getJwt,
};
