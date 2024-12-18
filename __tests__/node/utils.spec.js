import { describe, it, expect } from 'vitest';
import { removeTemplateEnginesSyntax } from '../../lib/utils';

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