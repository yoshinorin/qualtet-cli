const { logInfo, logError } = require("../../rust-lib/index.js");

async function waitForApiServerReady(url, timeoutSeconds = 120) {
  const startTime = Date.now();
  const timeout = timeoutSeconds * 1000;
  let attempt = 0;

  while (true) {
    attempt++;
    try {
      const response = await fetch(url);
      if (response.ok) {
        logInfo(`API server is ready at ${url}`);
        return true;
      }
    } catch (error) {
      // Nothing todo
    }

    const elapsed = Date.now() - startTime;
    if (elapsed >= timeout) {
      logError(`API server unavailable after ${timeoutSeconds} seconds`);
      process.exit(1);
    }
    logInfo(`Waiting for API server... (attempt ${attempt})`);
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}

module.exports = {
  waitForApiServerReady,
};
