/**
 * MorphLang — the shared language control.
 *
 * One picker, one stored preference, both pages. The studio's full 9-language
 * dictionary stays in index.html because it is specific to that page; what
 * lives here is only the chrome both pages share (navigation, copy buttons)
 * plus the plumbing: the language list, detection, persistence, and mounting.
 *
 * The guide's own prose lives in guide-i18n.js, which listens to the same
 * morphlang:change event. Both pages therefore speak all nine languages.
 *
 * Markup contract:
 *   <div class="lang-picker">
 *     <select id="lang-select"></select>        <- filled by mount()
 *     <span data-lang-code>EN</span>            <- kept in sync
 *   </div>
 *   Any element with data-chrome="key" gets CHROME[lang][key] as its text.
 */
(function (global) {
  'use strict';

  var STORAGE_KEY = 'morpheusicons.lang';

  var LANGS = [
    { code: 'en', native: 'English' },
    { code: 'pt', native: 'Português' },
    { code: 'es', native: 'Español' },
    { code: 'fr', native: 'Français' },
    { code: 'de', native: 'Deutsch' },
    { code: 'zh', native: '中文' },
    { code: 'ja', native: '日本語' },
    { code: 'ko', native: '한국어' },
    { code: 'ru', native: 'Русский' }
  ];

  // Copy/copied match index.html's own dictionary verbatim; the studio keeps
  // using its copy, this one serves the guide.
  var CHROME = {
    en: { skipToContent: "Skip to main content", footerLicense: "MorpheusIcons — MIT / Apache 2.0 License. Built for the Rust & Web UI community.", navStudio: 'Studio', navGetStarted: 'Get Started', copy: 'Copy', copied: 'Copied!', languageLabel: 'Language', guideBadge: 'Get Started Guide' },
    pt: { skipToContent: "Pular para o conteúdo principal", footerLicense: "MorpheusIcons — Licença MIT / Apache 2.0. Desenvolvido para a comunidade Rust & Web UI.", navStudio: 'Estúdio', navGetStarted: 'Começar', copy: 'Copiar', copied: 'Copiado!', languageLabel: 'Idioma', guideBadge: 'Guia de Introdução' },
    es: { skipToContent: "Saltar al contenido principal", footerLicense: "MorpheusIcons — Licencia MIT / Apache 2.0. Creado para la comunidad de Rust & Web UI.", navStudio: 'Estudio', navGetStarted: 'Empezar', copy: 'Copiar', copied: '¡Copiado!', languageLabel: 'Idioma', guideBadge: 'Guía de Inicio' },
    fr: { skipToContent: "Aller au contenu principal", footerLicense: "MorpheusIcons — Licence MIT / Apache 2.0. Conçu pour la communauté Rust & Web UI.", navStudio: 'Studio', navGetStarted: 'Démarrer', copy: 'Copier', copied: 'Copié !', languageLabel: 'Langue', guideBadge: 'Guide de Démarrage' },
    de: { skipToContent: "Zum Hauptinhalt springen", footerLicense: "MorpheusIcons — MIT / Apache 2.0 Lizenz. Entwickelt für die Rust & Web UI Community.", navStudio: 'Studio', navGetStarted: 'Loslegen', copy: 'Kopieren', copied: 'Kopiert!', languageLabel: 'Sprache', guideBadge: 'Einstiegs-Guide' },
    zh: { skipToContent: "跳到主要内容", footerLicense: "MorpheusIcons — MIT / Apache 2.0 开源协议。专为 Rust 与 Web UI 社区打造。", navStudio: '工作台', navGetStarted: '开始使用', copy: '复制', copied: '已复制!', languageLabel: '语言', guideBadge: '入门指南' },
    ja: { skipToContent: "メインコンテンツへスキップ", footerLicense: "MorpheusIcons — MIT / Apache 2.0 ライセンス。Rust & Web UI コミュニティのために開発。", navStudio: 'スタジオ', navGetStarted: 'はじめる', copy: 'コピー', copied: 'コピー完了!', languageLabel: '言語', guideBadge: 'スタートガイド' },
    ko: { skipToContent: "본문으로 건너뛰기", footerLicense: "MorpheusIcons — MIT / Apache 2.0 라이선스. Rust & Web UI 커뮤니티를 위해 제작되었습니다.", navStudio: '스튜디오', navGetStarted: '시작하기', copy: '복사', copied: '복사됨!', languageLabel: '언어', guideBadge: '시작 가이드' },
    ru: { skipToContent: "Перейти к основному содержанию", footerLicense: "MorpheusIcons — Лицензия MIT / Apache 2.0. Создано для сообщества Rust & Web UI.", navStudio: 'Студия', navGetStarted: 'Начать', copy: 'Копировать', copied: 'Скопировано!', languageLabel: 'Язык', guideBadge: 'Руководство' }
  };

  // Some browsers block storage entirely; a missing preference is not an error.
  function readStored() {
    try {
      return global.localStorage.getItem(STORAGE_KEY);
    } catch (e) {
      return null;
    }
  }

  function writeStored(code) {
    try {
      global.localStorage.setItem(STORAGE_KEY, code);
    } catch (e) {
      /* preference simply does not persist */
    }
  }

  function isKnown(code) {
    for (var i = 0; i < LANGS.length; i++) if (LANGS[i].code === code) return true;
    return false;
  }

  function detect() {
    var nav = global.navigator || {};
    var raw = (nav.language || nav.userLanguage || 'en').toLowerCase().split('-')[0];
    return isKnown(raw) ? raw : 'en';
  }

  var current = null;

  function get() {
    if (current) return current;
    var stored = readStored();
    current = stored && isKnown(stored) ? stored : detect();
    return current;
  }

  function htmlLangFor(code) {
    if (code === 'zh') return 'zh-CN';
    if (code === 'pt') return 'pt-BR';
    return code;
  }

  function t(key, code) {
    var dict = CHROME[code || get()] || CHROME.en;
    return dict[key] !== undefined ? dict[key] : CHROME.en[key];
  }

  /** Translate every [data-chrome] element under root. */
  function applyChrome(root, code) {
    var scope = root || document;
    var nodes = scope.querySelectorAll('[data-chrome]');
    for (var i = 0; i < nodes.length; i++) {
      nodes[i].textContent = t(nodes[i].dataset.chrome, code);
    }
  }

  function set(code, opts) {
    if (!isKnown(code)) code = 'en';
    current = code;
    writeStored(code);

    var faces = document.querySelectorAll('[data-lang-code]');
    for (var i = 0; i < faces.length; i++) faces[i].textContent = code.toUpperCase();

    var select = document.getElementById('lang-select');
    if (select && select.value !== code) select.value = code;

    var label = document.querySelector('[data-lang-label]');
    if (label) label.textContent = t('languageLabel', code);

    applyChrome(document, code);
    document.documentElement.lang = htmlLangFor(code);

    if (!opts || opts.notify !== false) {
      document.dispatchEvent(new CustomEvent('morphlang:change', { detail: { lang: code } }));
    }
  }

  function mount() {
    var select = document.getElementById('lang-select');
    if (select && !select.options.length) {
      var html = '';
      for (var i = 0; i < LANGS.length; i++) {
        html += '<option value="' + LANGS[i].code + '">' +
          LANGS[i].code.toUpperCase() + ' — ' + LANGS[i].native + '</option>';
      }
      select.innerHTML = html;
      select.addEventListener('change', function (e) { set(e.target.value); });
    }
    // notify:false — nothing has subscribed yet at mount time; each page
    // applies its own dictionary during boot.
    set(get(), { notify: false });
  }

  global.MorphLang = {
    LANGS: LANGS,
    CHROME: CHROME,
    get: get,
    set: set,
    t: t,
    mount: mount,
    applyChrome: applyChrome
  };

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', mount);
  } else {
    mount();
  }
})(window);
