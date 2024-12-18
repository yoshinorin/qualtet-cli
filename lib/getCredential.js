import { getCredential as getCredentialNative } from "../rust-lib/index.js";

export function getCredential(service, author) {
  return getCredentialNative(service, author);
}
