import { describe, it } from "vitest";
import { logInfo, logWarn, logError } from "../../rust-lib/index.js"

describe('logger', () => {
  it('should callable info level logger', () => {
    logInfo("info log test");
  });

  it('should callable warn level logger', () => {
    logWarn("warn log test");
  });

  it('should callable error level logger', () => {
    logError("error log test");
  });
}); 
