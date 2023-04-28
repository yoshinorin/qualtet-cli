const highlight = require('hexo-util').highlight

// from https://github.com/hexojs/hexo/blob/92b979f4a3fa8714aebd3d11c3295d466b870905/lib/plugins/filter/before_post_render/backtick_code_block.js
const rBacktick = /^((?:[^\S\r\n]*>){0,3}[^\S\r\n]*)(`{3,}|~{3,})[^\S\r\n]*((?:.*?[^`\s])?)[^\S\r\n]*\n((?:[\s\S]*?\n)?)(?:(?:[^\S\r\n]*>){0,3}[^\S\r\n]*)\2[^\S\r\n]?(\n+|$)/gm;
const rAllOptions = /([^\s]+)\s+(.+?)\s+(https?:\/\/\S+|\/\S+)\s*(.+)?/;
const rLangCaption = /([^\s]+)\s*(.+)?/;

function format(data) {
  if ((!data.includes('```') && !data.includes('~~~'))) {
    return data;
  }

  data = data.replace(rBacktick, ($0, start, $2, _args, _content, end) => {
    let content = _content.replace(/\n$/, '');

    // Extract language and caption of code blocks
    const args = _args.split('=').shift();
    let lang, caption;

    if (args) {
      const match = rAllOptions.exec(args) || rLangCaption.exec(args);

      if (match) {
        lang = match[1];

        if (match[2]) {
          caption = `<span>${match[2]}</span>`;

          if (match[3]) {
            caption += `<a href="${match[3]}">${match[4] ? match[4] : 'link'}</a>`;
          }
        }
      }
    }

    if (start.includes('>')) {
      // heading of last line is already removed by the top RegExp "rBacktick"
      const depth = start.split('>').length - 1;
      const regexp = new RegExp(`^([^\\S\\r\\n]*>){0,${depth}}([^\\S\\r\\n]|$)`, 'mg');
      content = content.replace(regexp, '');
    }

    // TODO: from config file
    const options = {
      hljs: false,
      autoDetect: false,
      gutter: true,
      tab: '',
      wrap: true,
      lang,
      caption
    };

    if (options.gutter) {
      first_line_number = false || 'always1';
      if (first_line_number === 'inline') {

        // setup line number by inline
        _args = _args.replace('=+', '=');
        options.gutter = _args.includes('=');

        // setup firstLineNumber;
        options.firstLine = options.gutter ? _args.split('=')[1] || 1 : 0;
      }
    }
    return start
      + '\n'
      + highlight(content, options)
      + '\n'
      + end;
  });
  return data;
}

module.exports = {
  format
};
