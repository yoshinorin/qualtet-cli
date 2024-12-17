import { getPassword } from "../rust-lib/index.js";

export function getCredential(service, author) {
  return getPassword(service, author);
}
