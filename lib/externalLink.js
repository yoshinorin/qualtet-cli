const { isExternalLink } = require('hexo-util');
const rATag = /<a(?:\s+?|\s+?[^<>]+?\s+?)href=["']((?:https?:|\/\/)[^<>"']+)["'][^<>]*>/gi;
const rTargetAttr = /target=/i;
const rRelAttr = /rel=/i;
const rRelStrAttr = /rel=["']([^<>"']*)["']/i;

// https://github.com/hexojs/hexo/blob/098cf0a517924e32e111e703bcac85ae632f3e0e/lib/plugins/filter/after_post_render/external_link.js
// https://github.com/hexojs/hexo-filter-nofollow/blob/9f7bcf417faf65292ee415a63ab342d947fe0eb6/lib/filter.js
function externalLink(data, url) {
  return data.replace(rATag, (str, href) => {
    if (!isExternalLink(href, url, []) || rTargetAttr.test(str)) return str;

    if (rRelAttr.test(str)) {
      str = str.replace(rRelStrAttr, (relStr, rel) => {
        return rel.includes('noopenner') ? relStr : `rel="${rel} noopener"`;
      });
      return str.replace('href=', 'target="_blank" href=');
    }

    return str.replace('href=', 'target="_blank" rel="noopener external nofollow noreferrer" href=');
  });
}

module.exports = {
  externalLink
};
