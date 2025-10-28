function findByPath(hexo, filePath) {
  const Post = hexo.model("Post");
  const post = Post.findOne({ source: filePath });
  if (post) {
    return { item: post, contentType: "article" };
  }

  const Page = hexo.model("Page");
  const page = Page.findOne({ source: filePath });
  if (page) {
    return { item: page, contentType: "page" };
  }
  return null;
}

module.exports = {
  findByPath,
};
