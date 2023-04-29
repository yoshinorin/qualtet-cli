const keytar = require('keytar');

function getCredential(service, author) {
  return keytar.getPassword(service, author);
}

module.exports = {
  getCredential
}