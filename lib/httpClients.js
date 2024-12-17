const axios = require("axios");

function httpClientWithNonAuth(apiUrl) {
  return axios.create({
    baseURL: apiUrl,
    headers: {
      "Content-Type": "application/json",
    },
  });
}

function httpClientWithAuth(apiUrl, token) {
  return axios.create({
    baseURL: apiUrl,
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
  });
}

module.exports = {
  httpClientWithNonAuth,
  httpClientWithAuth,
};
