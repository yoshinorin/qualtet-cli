const defaultHeadMeta = "noindex, noarchive, noimageindex, nofollow";

export function generateRobots(noindex, contentType) {
  if (contentType != "article") {
    return defaultHeadMeta;
  }

  if (noindex) {
    return defaultHeadMeta;
  }
  return "noarchive, noimageindex";
}
