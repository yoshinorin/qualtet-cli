const { parseArgs } = require("node:util");

function parseCommonArgs(additionalOptions = {}) {
  const { values } = parseArgs({
    options: {
      "api-url": { type: "string" },
      service: { type: "string" },
      author: { type: "string" },
      ...additionalOptions,
    },
  });

  return {
    apiUrl: values["api-url"],
    service: values.service,
    authorName: values.author,
    ...values,
  };
}

module.exports = { parseCommonArgs };
