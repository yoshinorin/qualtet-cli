import { setCredential as setPasswordNative } from "../rust-lib/index.js";
import * as readline from "node:readline";

const rline = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

rline.question(
  "Please input serviceName, authorName, and author's password: ",
  (x) => {
    const inputs = x.split(" ");
    setPasswordNative(inputs[0], inputs[1], inputs[2]);
    rline.close();
  },
);
