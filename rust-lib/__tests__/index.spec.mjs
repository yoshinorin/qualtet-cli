import { test, expect } from "vitest";
import { setCredential, getCredential } from "../index.js";

test("set and get password", () => {
  setCredential("hoge", "fuga", "bazbaaaaaz");
  const password = getCredential("hoge", "fuga");
  expect(password).toEqual("bazbaaaaaz");
});
