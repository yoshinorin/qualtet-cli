const fs = require("fs-extra");
const { join } = require("path");
const { isValidImage, logError, logInfo } = require("../../rust-lib/index.js");

function copyAssetsIfValid(assets, dest) {
  assets.forEach((a) => {
    if (isValidImage(a.source)) {
      const d = join(dest, a.path);
      fs.copy(a.source, d, (err) => {
        if (err) {
          logError(err);
        } else {
          logInfo(`Image copied - dest: ${d}`);
        }
      });
    } else {
      logError(`Image copy skipped - : ${a.path}`);
    }
  });
}

function copyContentAssets(item, options) {
  const { contentType, deployAssetsDir, hexo } = options;

  const assetModel =
    contentType === "article" ? hexo.model("PostAsset") : hexo.model("Asset");

  const assetDestPath =
    contentType === "article"
      ? join(hexo.base_dir, deployAssetsDir, "articles")
      : join(hexo.base_dir, deployAssetsDir);

  if (contentType === "article") {
    const assets = assetModel.find({ post: item._id }).toArray();
    copyAssetsIfValid(assets, assetDestPath);
  } else {
    const pageDir = item.path.slice(0, item.path.lastIndexOf("/"));
    const assets = assetModel.filter((x) => x._id.includes(pageDir));
    copyAssetsIfValid(assets, assetDestPath);
  }
}

module.exports = {
  copyContentAssets,
};
