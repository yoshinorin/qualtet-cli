const defaultHeadMeta = "noindex, noarchive, noimageindex, nofollow";

/**
 * @deprecated
 */
export function generateRobots(noindex, contentType) {
  if (contentType != "article") {
    return defaultHeadMeta;
  }

  if (noindex) {
    return defaultHeadMeta;
  }
  return "noarchive, noimageindex";
}
