// NOTE: This package has been archived...
//       I have to migrate to another package or another language.
const keytar = require('keytar');
const readline = require('readline').createInterface({
  input: process.stdin,
  output: process.stdout
});

readline.question("Please input serviceName, authorName, and author's password: ", (x) => {
  const inputs = x.split(' ');
  keytar.setPassword(inputs[0], inputs[1], inputs[2]);
  readline.close();
});
