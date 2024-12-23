import { describe, it, expect } from 'vitest';
import { removeTemplateEnginesSyntax, formatPath } from '../../lib/utils';

describe('removeTemplateEnginesSyntax', () => {
  it('should remove {% raw %} and {% endraw %} from the text', () => {
    const input = "This is a {% raw %}test{% endraw %} string.";
    const expectedOutput = "This is a test string.";
    expect(removeTemplateEnginesSyntax(input)).toBe(expectedOutput);
  });

  it('should return the same text if there are no template engine syntax', () => {
    const input = "This is a test string.";
    const expectedOutput = "This is a test string.";
    expect(removeTemplateEnginesSyntax(input)).toBe(expectedOutput);
  });

  it('should handle multiple occurrences of template engine syntax', () => {
    const input = "{% raw %}This{% endraw %} is a {% raw %}test{% endraw %} string.";
    const expectedOutput = "This is a test string.";
    expect(removeTemplateEnginesSyntax(input)).toBe(expectedOutput);
  });

  it('should handle empty strings', () => {
    const input = "";
    const expectedOutput = "";
    expect(removeTemplateEnginesSyntax(input)).toBe(expectedOutput);
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