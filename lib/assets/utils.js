const fs = require("fs-extra");
const { join } = require("path");
const { isValidImage, logError } = require("../../rust-lib/index.js");

function copyAssetsIfValid(assets, dest) {
  assets.forEach((a) => {
    if (isValidImage(a.source)) {
      fs.copy(a.source, join(dest, a.path), (err) => {
        if (err) {
          logError(err);
        }
      });
    } else {
      logError(`image copy skipped - : ${a.path}`);
    }
  });
}

module.exports = {
  copyAssetsIfValid,
};
