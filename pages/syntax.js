/**
 * MorphSyntax — dependency-free syntax highlighting for the code panels.
 *
 * MorpheusIcons ships no runtime JS dependencies, so this replaces a
 * highlight.js/Prism CDN tag with ~120 lines of tokenizer. It covers only the
 * four languages the site actually shows: Rust, JavaScript, TOML, and shell.
 *
 * Colours live in src/input.css (.tok-*), not here, so the palette stays in
 * the design system rather than in a script.
 *
 * Usage:
 *   MorphSyntax.highlight(rawCode, 'rust')  -> escaped HTML string
 *   MorphSyntax.apply()                     -> highlights every
 *                                              <pre><code data-lang="…">
 */
(function (global) {
  'use strict';

  var KEYWORDS = {
    rust: 'as async await break const continue crate dyn else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while',
    js: 'as async await break case catch class const continue default delete do else export extends finally for from function if import in instanceof let new of return static super switch this throw try typeof var void while yield true false null undefined',
    toml: 'true false',
    sh: 'cargo npm npx pnpm yarn curl wasm-pack make cd echo git'
  };

  Object.keys(KEYWORDS).forEach(function (k) {
    KEYWORDS[k] = KEYWORDS[k].split(' ').reduce(function (set, w) {
      set[w] = true;
      return set;
    }, Object.create(null));
  });

  // Every inner group must be non-capturing: the tokenizer maps outer group
  // index back to a rule, so a stray capture would shift the whole mapping.
  var COMMON = {
    lineComment: { re: '\\/\\/[^\\n]*', cls: 'tok-com' },
    blockComment: { re: '\\/\\*[\\s\\S]*?\\*\\/', cls: 'tok-com' },
    hashComment: { re: '#[^\\n]*', cls: 'tok-com' },
    dquote: { re: '"(?:\\\\.|[^"\\\\])*"', cls: 'tok-str' },
    squote: { re: "'(?:\\\\.|[^'\\\\])*'", cls: 'tok-str' },
    backtick: { re: '`(?:\\\\.|[^`\\\\])*`', cls: 'tok-str' },
    punct: { re: '[{}()\\[\\];,.:<>=+\\-*/&|!?%]+', cls: 'tok-pun' }
  };

  var GRAMMARS = {
    rust: [
      COMMON.lineComment,
      COMMON.blockComment,
      { re: '#!?\\[[^\\]]*\\]', cls: 'tok-att' },
      { re: 'r#*"(?:[^"]|"(?!#))*"#*', cls: 'tok-str' },
      COMMON.dquote,
      { re: "'(?:\\\\.|[^'\\\\])'", cls: 'tok-str' },
      { re: "'[A-Za-z_][A-Za-z0-9_]*", cls: 'tok-att' },
      { re: '[A-Za-z_][A-Za-z0-9_]*!', cls: 'tok-mac' },
      { re: '\\b\\d[\\d_]*(?:\\.\\d+)?(?:[iuf](?:8|16|32|64|128|size))?\\b', cls: 'tok-num' },
      { re: '\\b[A-Z][A-Za-z0-9_]*\\b', cls: 'tok-typ' },
      { re: '\\b[A-Za-z_][A-Za-z0-9_]*\\b', cls: 'word' },
      COMMON.punct
    ],
    js: [
      COMMON.lineComment,
      COMMON.blockComment,
      COMMON.dquote,
      COMMON.squote,
      COMMON.backtick,
      { re: '\\b\\d[\\d_]*(?:\\.\\d+)?\\b', cls: 'tok-num' },
      { re: '\\b[A-Z][A-Za-z0-9_]*\\b', cls: 'tok-typ' },
      { re: '\\b[A-Za-z_$][A-Za-z0-9_$]*\\b', cls: 'word' },
      COMMON.punct
    ],
    toml: [
      COMMON.hashComment,
      { re: '^\\s*\\[[^\\]\\n]*\\]', cls: 'tok-typ' },
      COMMON.dquote,
      COMMON.squote,
      { re: '\\b\\d[\\d_]*(?:\\.\\d+)?\\b', cls: 'tok-num' },
      { re: '\\b[A-Za-z_][A-Za-z0-9_-]*\\b', cls: 'word' },
      COMMON.punct
    ],
    sh: [
      COMMON.hashComment,
      COMMON.dquote,
      COMMON.squote,
      { re: '(?:^|\\s)--?[A-Za-z][A-Za-z0-9-]*', cls: 'tok-att' },
      { re: '\\b[A-Za-z_][A-Za-z0-9_-]*\\b', cls: 'word' },
      COMMON.punct
    ]
  };

  var compiled = Object.create(null);

  function grammarFor(lang) {
    var key = GRAMMARS[lang] ? lang : 'rust';
    if (!compiled[key]) {
      var rules = GRAMMARS[key];
      compiled[key] = {
        rules: rules,
        re: new RegExp(rules.map(function (r) { return '(' + r.re + ')'; }).join('|'), 'gm'),
        keywords: KEYWORDS[key] || KEYWORDS.rust
      };
    }
    return compiled[key];
  }

  function esc(s) {
    return s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }

  function highlight(code, lang) {
    var g = grammarFor(lang);
    var re = new RegExp(g.re.source, g.re.flags);
    var out = '';
    var last = 0;
    var m;

    while ((m = re.exec(code)) !== null) {
      // Zero-length match would spin forever; step past it.
      if (m[0] === '') { re.lastIndex++; continue; }
      if (m.index > last) out += esc(code.slice(last, m.index));

      var cls = null;
      for (var i = 0; i < g.rules.length; i++) {
        if (m[i + 1] !== undefined) { cls = g.rules[i].cls; break; }
      }
      if (cls === 'word') cls = g.keywords[m[0]] ? 'tok-key' : null;

      out += cls ? '<span class="' + cls + '">' + esc(m[0]) + '</span>' : esc(m[0]);
      last = m.index + m[0].length;
    }

    return out + esc(code.slice(last));
  }

  function apply(root) {
    var scope = root || document;
    var nodes = scope.querySelectorAll('pre > code[data-lang]');
    for (var i = 0; i < nodes.length; i++) {
      var el = nodes[i];
      if (el.dataset.highlighted === 'true') continue;
      el.innerHTML = highlight(el.textContent, el.dataset.lang);
      el.dataset.highlighted = 'true';
    }
  }

  global.MorphSyntax = { highlight: highlight, apply: apply };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function () { apply(); });
  } else {
    apply();
  }
})(window);
