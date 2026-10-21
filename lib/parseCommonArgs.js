const { parseArgs } = require("node:util");
const { setIpv4Only } = require("../rust-lib/index.js");

function parseCommonArgs(additionalOptions = {}) {
  const { values } = parseArgs({
    options: {
      "api-url": { type: "string" },
      service: { type: "string" },
      author: { type: "string" },
      ipv4: { type: "boolean", default: false },
      ...additionalOptions,
    },
  });

  setIpv4Only(values.ipv4);

  return {
    apiUrl: values["api-url"],
    service: values.service,
    authorName: values.author,
    ...values,
  };
}

module.exports = { parseCommonArgs };
