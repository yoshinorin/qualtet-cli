const Hexo = require("hexo");
const hexo = new Hexo(process.cwd(), { silent: false });
const { logInfo, isValidImage } = require("../rust-lib/index.js");
const { parseArgs } = require("node:util");

const { values } = parseArgs({
  options: {
    "days-ago": { type: "string", default: "10000" },
  },
});

const daysAgo = values["days-ago"];

logInfo(`check updated in ${daysAgo} days ago articles assets.`);

function assertImages(assets) {
  assets.forEach((a) => {
    isValidImage(a.source);
  });
}

hexo.init().then(() => {
  hexo.load().then(() => {
    let date = new Date();
    date = date.setDate(date.getDate() - daysAgo);
    const postAsset = hexo.model("PostAsset");
    const pageAsset = hexo.model("Asset");

    const posts = hexo.locals.get("posts").filter((c) => c.updated > date);
    for (let post of posts.toArray()) {
      const assets = postAsset.find({ post: post._id }).toArray();
      assertImages(assets);
    }

    const pages = hexo.locals.get("pages").filter((c) => c.updated > date);
    for (let page of pages.toArray()) {
      const pageDir = page.path.slice(0, page.path.lastIndexOf("/"));
      const assets = pageAsset.filter((x) => x._id.includes(pageDir));
      assertImages(assets);
    }
  });
});
