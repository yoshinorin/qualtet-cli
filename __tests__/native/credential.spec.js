import { expect, describe, it } from "vitest";
import { setCredential, getCredential } from "../../rust-lib/index.js"

describe("Credential Tests", () => {
  it("should set and get password", () => {
    setCredential("hoge", "fuga", "bazbaaaaaz");
    const password = getCredential("hoge", "fuga");
    expect(password).toEqual("bazbaaaaaz");
  });
});
