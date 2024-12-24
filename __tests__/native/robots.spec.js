import { expect, describe, it } from "vitest";
import { generateRobots } from "../../rust-lib/index.js"

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
