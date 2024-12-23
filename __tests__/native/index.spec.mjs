import { test, expect, describe, it } from "vitest";
import { setCredential, getCredential, removeTemplateEnginesSyntax, generateRobots, formatPath } from "../../rust-lib/index.js"
import { test, expect, describe, it } from "vitest";

describe("Credential Tests", () => {
  it("should set and get password", () => {
    setCredential("hoge", "fuga", "bazbaaaaaz");
    const password = getCredential("hoge", "fuga");
    expect(password).toEqual("bazbaaaaaz");
  });
});

describe("removeTemplateEnginesSyntax Tests", () => {
  it("should remove template syntax", () => {
    const input = "Hello {% raw %}, welcome to {% endraw %}!";
    const expectedOutput = "Hello , welcome to !";
    const result = removeTemplateEnginesSyntax(input);
    expect(result).toEqual(expectedOutput);
  });

  it("should handle strings with no template syntax", () => {
    const input = "Hello world!";
    const expectedOutput = "Hello world!";
    const result = removeTemplateEnginesSyntax(input);
    expect(result).toEqual(expectedOutput);
  });


  it("should handle empty strings", () => {
    const input = "";
    const expectedOutput = "";
    const result = removeTemplateEnginesSyntax(input);
    expect(result).toEqual(expectedOutput);
  });

  it("should handle strings with only template syntax", () => {
    const input = "{% raw %}{% endraw %}";
    const expectedOutput = "";
    const result = removeTemplateEnginesSyntax(input);
    expect(result).toEqual(expectedOutput);
  });
});

describe('formatPath', () => {
  it('should return the same path if it ends with a slash', () => {
    const inputPath = "/example/path/";
    const contentType = "article";
    const expectedOutput = "/articles/example/path/";
    expect(formatPath(inputPath, contentType)).toBe(expectedOutput);
  });

  it('should add a slash at the end if the path does not end with a slash', () => {
    const inputPath = "/example/path";
    const contentType = "article";
    const expectedOutput = "/articles/example/path/";
    expect(formatPath(inputPath, contentType)).toBe(expectedOutput);
  });

  it('should add a leading slash if the path does not start with a slash', () => {
    const inputPath = "example/path/";
    const contentType = "article";
    const expectedOutput = "/articles/example/path/";
    expect(formatPath(inputPath, contentType)).toBe(expectedOutput);
  });

  it('should add "/articles" to the path if contentType is "article" and path does not include "/articles"', () => {
    const inputPath = "/example/path/";
    const contentType = "article";
    const expectedOutput = "/articles/example/path/";
    expect(formatPath(inputPath, contentType)).toBe(expectedOutput);
  });

  it('should not add "/articles" to the path if contentType is not "article"', () => {
    const inputPath = "/example/path/";
    const contentType = "page";
    const expectedOutput = "/example/path/";
    expect(formatPath(inputPath, contentType)).toBe(expectedOutput);
  });

  // TODO: throw exception
  it('should handle empty paths', () => {
    const inputPath = "";
    const contentType = "article";
    const expectedOutput = "/articles/";
    expect(formatPath(inputPath, contentType)).toBe(expectedOutput);
  });
});

describe('generateRobots', () => {
  const defaultHeadMeta = "noindex, noarchive, noimageindex, nofollow";

  it('should return defaultHeadMeta if contentType is not "article"', () => {
    expect(generateRobots(false, "page")).toBe(defaultHeadMeta);
    expect(generateRobots(true, "page")).toBe(defaultHeadMeta);
  });

  it('should return defaultHeadMeta if noindex is true', () => {
    expect(generateRobots(true, "article")).toBe(defaultHeadMeta);
  });

  it('should return "noarchive, noimageindex" if contentType is "article" and noindex is false', () => {
    expect(generateRobots(false, "article")).toBe("noarchive, noimageindex");
  });

  it('should return "noarchive, noimageindex" if contentType is "article" and noindex is null', () => {
    expect(generateRobots(null, "article")).toBe("noarchive, noimageindex");
  });
});
