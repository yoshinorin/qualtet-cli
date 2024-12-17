const Hexo = require("hexo");
const hexo = new Hexo(process.cwd(), { silent: false });
const log = require("hexo-log").default({
  debug: false,
  silent: false,
});
const { validate } = require("../lib/imageValidation");

const daysAgo = process.argv[2] ? process.argv[2] : 10000;

log.info(`check updated in ${daysAgo} days ago articles assets.`);

hexo.init().then(() => {
  hexo.load().then(() => {
    let date = new Date();
    date = date.setDate(date.getDate() - daysAgo);
    const postAsset = hexo.model("PostAsset");
    const pageAsset = hexo.model("Asset");

    const posts = hexo.locals.get("posts").filter((c) => c.updated > date);
    for (let post of posts.toArray()) {
      const assets = postAsset.find({ post: post._id }).toArray();
      assets.forEach((a) => {
        if (
          a.source.endsWith(".pptx") ||
          a.source.endsWith(".svg") ||
          a.source.endsWith(".ico") ||
          a.source.endsWith(".mp3") ||
          a.source.endsWith(".gif")
        ) {
          // Nothing todo
        } else {
          validate(a.source);
        }
      });
    }

    const pages = hexo.locals.get("pages").filter((c) => c.updated > date);
    for (let page of pages.toArray()) {
      const pageDir = page.path.slice(0, page.path.lastIndexOf("/"));
      const assets = pageAsset.filter((x) => x._id.includes(pageDir));
      assets.forEach((a) => {
        if (
          a.source.endsWith(".pptx") ||
          a.source.endsWith(".svg") ||
          a.source.endsWith(".ico") ||
          a.source.endsWith(".mp3") ||
          a.source.endsWith(".gif")
        ) {
          // Nothing todo
        } else {
          validate(a.source);
        }
      });
    }
  });
});
