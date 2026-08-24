/**
 * Get Started guide — prose in the same nine languages as the studio.
 *
 * Values are innerHTML, not plain text, so each language can put the inline
 * <code> chips where its own grammar wants them instead of being forced into
 * English word order.
 *
 * What is NOT translated: the code snippets, including their comments. The
 * Copy button must hand every reader the same tested code, and an identifier
 * or comment that differs per locale is a support burden, not a courtesy.
 * Framework names (GPUI, egui, Leptos), crate names and CDN names stay as
 * written for the same reason.
 */
(function (global) {
  'use strict';

  var C = 'class="gcode"';
  var K = 'class="gcode gcode-key"';
  var M = 'class="font-mono"';
  var T = 'class="text-green-700 font-mono text-sm"';
  var LINK = '<a href="https://lucide.dev/icons" target="_blank" rel="noopener noreferrer" class="text-green-700 font-bold hover:underline">lucide.dev/icons</a>';

  var GUIDE = {
    en: {
      badge: 'Bring Your Own Icons — icondata, Lucide, Heroicons, Tabler &amp; Phosphor',
      heroTitle: 'Using <span class="text-green-700">Lucide via Cargo, npm or lucide.dev</span> in Any Project',
      heroDesc: 'MorpheusIcons works with <strong>any icon library from Cargo crates, npm, or lucide.dev</strong>. Start with the Rust crate <code ' + C + '>icondata</code>, or load SVGs from npm packages like <code ' + C + '>lucide-static</code> and fetch them from a CDN.',
      jumpLabel: 'Jump to:',
      jump1: 'Rust icondata crate',
      jump4: 'GUI Frameworks',
      s1Title: '1. Using Lucide in Rust via Cargo (<code ' + M + '>icondata</code>)',
      s1Sub: 'Using the <code ' + M + '>icondata</code> Rust crate or <code ' + M + '>lucide.dev</code> site SVGs',
      s1aTitle: 'Option A: Using the <code ' + T + '>icondata</code> Crate (Lucide, Heroicons, Tabler for Rust)',
      s1aIntro: 'In Rust, the popular <code ' + K + '>icondata</code> crate packages Lucide, Heroicons, Tabler, and Phosphor icons into Rust constants:',
      s1aPass: 'Pass <code ' + C + '>icondata</code> path data directly into MorpheusIcons:',
      s1bTitle: 'Option B: Copying Directly from <code ' + T + '>lucide.dev</code>',
      s1bSteps: '<li>Open ' + LINK + '</li><li>Search for any icon (for example <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li>Click <strong>Copy SVG</strong>, or save the <code ' + C + '>.svg</code> file into your project</li><li>Pass it through <code ' + K + '>icon_from_svg</code> to convert the full SVG</li>',
      s2Title: '2. Using Lucide from npm in JavaScript / Web Apps',
      s2Sub: 'Integrating <code ' + M + '>lucide-static</code> or Lucide CDN with MorpheusIcons WebAssembly',
      s2aTitle: 'Option A: Using the <code ' + T + '>lucide-static</code> npm Package',
      s2aIntro: 'Install the official <code ' + C + '>lucide-static</code> package in your web project (Vite, Next.js, Webpack, etc.):',
      s2aThen: 'Then import Lucide icon paths and feed them directly into MorpheusIcons:',
      s2bTitle: 'Option B: Fetching Lucide Icons Dynamically from CDN (unpkg / jsDelivr)',
      s2bIntro: 'No build tools required! You can fetch any icon dynamically from Lucide’s CDN:',
      s3Title: 'GUI Framework Integration Examples',
      s3Sub: 'Ready-to-copy code snippets for GPUI, egui, Iced, Leptos, Dioxus &amp; WASM',
      fwGpui: 'GPUI (Zed Editor Framework)',
      fwEgui: 'egui Immediate-Mode UI',
      fwLeptos: 'Leptos Reactive Web Components'
    },

    pt: {
      badge: 'Use seus próprios ícones — icondata, Lucide, Heroicons, Tabler e Phosphor',
      heroTitle: 'Usando <span class="text-green-700">Lucide via Cargo, npm ou lucide.dev</span> em qualquer projeto',
      heroDesc: 'O MorpheusIcons funciona com <strong>qualquer biblioteca de ícones vinda de crates do Cargo, do npm ou do lucide.dev</strong>. Comece pelo crate Rust <code ' + C + '>icondata</code>, ou carregue SVGs de pacotes npm como <code ' + C + '>lucide-static</code> e busque-os por CDN.',
      jumpLabel: 'Ir para:',
      jump1: 'Crate icondata (Rust)',
      jump4: 'Frameworks de GUI',
      s1Title: '1. Usando Lucide em Rust via Cargo (<code ' + M + '>icondata</code>)',
      s1Sub: 'Usando o crate Rust <code ' + M + '>icondata</code> ou os SVGs do site <code ' + M + '>lucide.dev</code>',
      s1aTitle: 'Opção A: usando o crate <code ' + T + '>icondata</code> (Lucide, Heroicons e Tabler para Rust)',
      s1aIntro: 'Em Rust, o popular crate <code ' + K + '>icondata</code> empacota os ícones do Lucide, Heroicons, Tabler e Phosphor como constantes Rust:',
      s1aPass: 'Passe os dados de path do <code ' + C + '>icondata</code> direto para o MorpheusIcons:',
      s1bTitle: 'Opção B: copiando direto do <code ' + T + '>lucide.dev</code>',
      s1bSteps: '<li>Abra ' + LINK + '</li><li>Procure qualquer ícone (por exemplo <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li>Clique em <strong>Copy SVG</strong>, ou salve o arquivo <code ' + C + '>.svg</code> no seu projeto</li><li>Passe-o por <code ' + K + '>icon_from_svg</code> para converter o SVG completo</li>',
      s2Title: '2. Usando Lucide do npm em JavaScript / aplicações web',
      s2Sub: 'Integrando <code ' + M + '>lucide-static</code> ou a CDN do Lucide com o MorpheusIcons WebAssembly',
      s2aTitle: 'Opção A: usando o pacote npm <code ' + T + '>lucide-static</code>',
      s2aIntro: 'Instale o pacote oficial <code ' + C + '>lucide-static</code> no seu projeto web (Vite, Next.js, Webpack, etc.):',
      s2aThen: 'Depois importe os paths dos ícones do Lucide e passe-os direto para o MorpheusIcons:',
      s2bTitle: 'Opção B: buscando ícones do Lucide dinamicamente por CDN (unpkg / jsDelivr)',
      s2bIntro: 'Sem nenhuma ferramenta de build! Você pode buscar qualquer ícone dinamicamente pela CDN do Lucide:',
      s3Title: 'Exemplos de integração com frameworks de GUI',
      s3Sub: 'Trechos de código prontos para copiar para GPUI, egui, Iced, Leptos, Dioxus e WASM',
      fwGpui: 'GPUI (framework do editor Zed)',
      fwEgui: 'egui, interface em modo imediato',
      fwLeptos: 'Leptos, componentes web reativos'
    },

    es: {
      badge: 'Usa tus propios iconos — icondata, Lucide, Heroicons, Tabler y Phosphor',
      heroTitle: 'Usando <span class="text-green-700">Lucide con Cargo, npm o lucide.dev</span> en cualquier proyecto',
      heroDesc: 'MorpheusIcons funciona con <strong>cualquier biblioteca de iconos de crates de Cargo, npm o lucide.dev</strong>. Empieza por el crate de Rust <code ' + C + '>icondata</code>, o carga SVG desde paquetes npm como <code ' + C + '>lucide-static</code> y obtenlos desde una CDN.',
      jumpLabel: 'Ir a:',
      jump1: 'Crate icondata (Rust)',
      jump4: 'Frameworks de GUI',
      s1Title: '1. Usar Lucide en Rust con Cargo (<code ' + M + '>icondata</code>)',
      s1Sub: 'Usando el crate de Rust <code ' + M + '>icondata</code> o los SVG del sitio <code ' + M + '>lucide.dev</code>',
      s1aTitle: 'Opción A: usar el crate <code ' + T + '>icondata</code> (Lucide, Heroicons y Tabler para Rust)',
      s1aIntro: 'En Rust, el popular crate <code ' + K + '>icondata</code> empaqueta los iconos de Lucide, Heroicons, Tabler y Phosphor como constantes de Rust:',
      s1aPass: 'Pasa los datos de path de <code ' + C + '>icondata</code> directamente a MorpheusIcons:',
      s1bTitle: 'Opción B: copiar directamente desde <code ' + T + '>lucide.dev</code>',
      s1bSteps: '<li>Abre ' + LINK + '</li><li>Busca cualquier icono (por ejemplo <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li>Haz clic en <strong>Copy SVG</strong>, o guarda el archivo <code ' + C + '>.svg</code> en tu proyecto</li><li>Pásalo por <code ' + K + '>icon_from_svg</code> para convertir el SVG completo</li>',
      s2Title: '2. Usar Lucide desde npm en JavaScript / aplicaciones web',
      s2Sub: 'Integrando <code ' + M + '>lucide-static</code> o la CDN de Lucide con MorpheusIcons WebAssembly',
      s2aTitle: 'Opción A: usar el paquete npm <code ' + T + '>lucide-static</code>',
      s2aIntro: 'Instala el paquete oficial <code ' + C + '>lucide-static</code> en tu proyecto web (Vite, Next.js, Webpack, etc.):',
      s2aThen: 'Después importa los paths de los iconos de Lucide y pásalos directamente a MorpheusIcons:',
      s2bTitle: 'Opción B: obtener iconos de Lucide dinámicamente desde una CDN (unpkg / jsDelivr)',
      s2bIntro: '¡Sin herramientas de compilación! Puedes obtener cualquier icono dinámicamente desde la CDN de Lucide:',
      s3Title: 'Ejemplos de integración con frameworks de GUI',
      s3Sub: 'Fragmentos de código listos para copiar para GPUI, egui, Iced, Leptos, Dioxus y WASM',
      fwGpui: 'GPUI (framework del editor Zed)',
      fwEgui: 'egui, interfaz en modo inmediato',
      fwLeptos: 'Leptos, componentes web reactivos'
    },

    fr: {
      badge: 'Apportez vos propres icônes — icondata, Lucide, Heroicons, Tabler et Phosphor',
      heroTitle: 'Utiliser <span class="text-green-700">Lucide via Cargo, npm ou lucide.dev</span> dans n’importe quel projet',
      heroDesc: 'MorpheusIcons fonctionne avec <strong>n’importe quelle bibliothèque d’icônes issue des crates Cargo, de npm ou de lucide.dev</strong>. Commencez par la crate Rust <code ' + C + '>icondata</code>, ou chargez des SVG depuis des paquets npm comme <code ' + C + '>lucide-static</code> et récupérez-les via un CDN.',
      jumpLabel: 'Aller à :',
      jump1: 'Crate icondata (Rust)',
      jump4: 'Frameworks GUI',
      s1Title: '1. Utiliser Lucide en Rust via Cargo (<code ' + M + '>icondata</code>)',
      s1Sub: 'Avec la crate Rust <code ' + M + '>icondata</code> ou les SVG du site <code ' + M + '>lucide.dev</code>',
      s1aTitle: 'Option A : utiliser la crate <code ' + T + '>icondata</code> (Lucide, Heroicons, Tabler pour Rust)',
      s1aIntro: 'En Rust, la populaire crate <code ' + K + '>icondata</code> regroupe les icônes Lucide, Heroicons, Tabler et Phosphor sous forme de constantes Rust :',
      s1aPass: 'Passez les données de tracé d’<code ' + C + '>icondata</code> directement à MorpheusIcons :',
      s1bTitle: 'Option B : copier directement depuis <code ' + T + '>lucide.dev</code>',
      s1bSteps: '<li>Ouvrez ' + LINK + '</li><li>Cherchez n’importe quelle icône (par exemple <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li>Cliquez sur <strong>Copy SVG</strong>, ou enregistrez le fichier <code ' + C + '>.svg</code> dans votre projet</li><li>Passez-le à <code ' + K + '>icon_from_svg</code> pour convertir le SVG complet</li>',
      s2Title: '2. Utiliser Lucide depuis npm en JavaScript / applications web',
      s2Sub: 'Intégrer <code ' + M + '>lucide-static</code> ou le CDN Lucide avec MorpheusIcons WebAssembly',
      s2aTitle: 'Option A : utiliser le paquet npm <code ' + T + '>lucide-static</code>',
      s2aIntro: 'Installez le paquet officiel <code ' + C + '>lucide-static</code> dans votre projet web (Vite, Next.js, Webpack, etc.) :',
      s2aThen: 'Importez ensuite les tracés des icônes Lucide et passez-les directement à MorpheusIcons :',
      s2bTitle: 'Option B : récupérer les icônes Lucide dynamiquement depuis un CDN (unpkg / jsDelivr)',
      s2bIntro: 'Aucun outil de build requis ! Vous pouvez récupérer n’importe quelle icône dynamiquement depuis le CDN de Lucide :',
      s3Title: 'Exemples d’intégration aux frameworks GUI',
      s3Sub: 'Extraits de code prêts à copier pour GPUI, egui, Iced, Leptos, Dioxus et WASM',
      fwGpui: 'GPUI (framework de l’éditeur Zed)',
      fwEgui: 'egui, interface en mode immédiat',
      fwLeptos: 'Leptos, composants web réactifs'
    },

    de: {
      badge: 'Bring deine eigenen Icons mit — icondata, Lucide, Heroicons, Tabler und Phosphor',
      heroTitle: '<span class="text-green-700">Lucide über Cargo, npm oder lucide.dev</span> in jedem Projekt verwenden',
      heroDesc: 'MorpheusIcons arbeitet mit <strong>jeder Icon-Bibliothek aus Cargo-Crates, npm oder lucide.dev</strong>. Beginne mit dem Rust-Crate <code ' + C + '>icondata</code>, oder lade SVGs aus npm-Paketen wie <code ' + C + '>lucide-static</code> und hole sie über ein CDN.',
      jumpLabel: 'Springe zu:',
      jump1: 'icondata-Crate (Rust)',
      jump4: 'GUI-Frameworks',
      s1Title: '1. Lucide in Rust über Cargo verwenden (<code ' + M + '>icondata</code>)',
      s1Sub: 'Mit dem Rust-Crate <code ' + M + '>icondata</code> oder den SVGs von <code ' + M + '>lucide.dev</code>',
      s1aTitle: 'Variante A: das Crate <code ' + T + '>icondata</code> verwenden (Lucide, Heroicons, Tabler für Rust)',
      s1aIntro: 'In Rust bündelt das verbreitete Crate <code ' + K + '>icondata</code> die Icons von Lucide, Heroicons, Tabler und Phosphor als Rust-Konstanten:',
      s1aPass: 'Übergib die Pfaddaten aus <code ' + C + '>icondata</code> direkt an MorpheusIcons:',
      s1bTitle: 'Variante B: direkt von <code ' + T + '>lucide.dev</code> kopieren',
      s1bSteps: '<li>Öffne ' + LINK + '</li><li>Suche ein beliebiges Icon (zum Beispiel <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li>Klicke auf <strong>Copy SVG</strong> oder speichere die <code ' + C + '>.svg</code>-Datei in deinem Projekt</li><li>Gib sie an <code ' + K + '>icon_from_svg</code> weiter, um das vollständige SVG zu konvertieren</li>',
      s2Title: '2. Lucide aus npm in JavaScript / Web-Apps verwenden',
      s2Sub: '<code ' + M + '>lucide-static</code> oder das Lucide-CDN mit MorpheusIcons WebAssembly verbinden',
      s2aTitle: 'Variante A: das npm-Paket <code ' + T + '>lucide-static</code> verwenden',
      s2aIntro: 'Installiere das offizielle Paket <code ' + C + '>lucide-static</code> in deinem Web-Projekt (Vite, Next.js, Webpack usw.):',
      s2aThen: 'Importiere dann die Lucide-Icon-Pfade und übergib sie direkt an MorpheusIcons:',
      s2bTitle: 'Variante B: Lucide-Icons dynamisch über ein CDN laden (unpkg / jsDelivr)',
      s2bIntro: 'Keine Build-Tools nötig! Du kannst jedes Icon dynamisch über das Lucide-CDN laden:',
      s3Title: 'Integrationsbeispiele für GUI-Frameworks',
      s3Sub: 'Fertige Code-Schnipsel zum Kopieren für GPUI, egui, Iced, Leptos, Dioxus und WASM',
      fwGpui: 'GPUI (Framework des Zed-Editors)',
      fwEgui: 'egui, Immediate-Mode-Oberfläche',
      fwLeptos: 'Leptos, reaktive Web-Komponenten'
    },

    zh: {
      badge: '自带图标库 — icondata、Lucide、Heroicons、Tabler 与 Phosphor',
      heroTitle: '在任意项目中<span class="text-green-700">通过 Cargo、npm 或 lucide.dev 使用 Lucide</span>',
      heroDesc: 'MorpheusIcons 可以配合 <strong>来自 Cargo crate、npm 或 lucide.dev 的任意图标库</strong> 使用。可以从 Rust crate <code ' + C + '>icondata</code> 开始，也可以从 <code ' + C + '>lucide-static</code> 这类 npm 包加载 SVG，或直接从 CDN 获取。',
      jumpLabel: '跳转到：',
      jump1: 'Rust icondata crate',
      jump4: 'GUI 框架',
      s1Title: '1. 通过 Cargo 在 Rust 中使用 Lucide（<code ' + M + '>icondata</code>）',
      s1Sub: '使用 Rust crate <code ' + M + '>icondata</code>，或 <code ' + M + '>lucide.dev</code> 站点的 SVG',
      s1aTitle: '方式 A：使用 <code ' + T + '>icondata</code> crate（面向 Rust 的 Lucide、Heroicons、Tabler）',
      s1aIntro: '在 Rust 中，常用的 <code ' + K + '>icondata</code> crate 把 Lucide、Heroicons、Tabler 和 Phosphor 的图标打包成 Rust 常量：',
      s1aPass: '把 <code ' + C + '>icondata</code> 的路径数据直接传给 MorpheusIcons：',
      s1bTitle: '方式 B：直接从 <code ' + T + '>lucide.dev</code> 复制',
      s1bSteps: '<li>打开 ' + LINK + '</li><li>搜索任意图标（例如 <code ' + C + '>play</code>、<code ' + C + '>pause</code>、<code ' + C + '>sun</code>）</li><li>点击 <strong>Copy SVG</strong>，或把 <code ' + C + '>.svg</code> 文件保存到项目中</li><li>用 <code ' + K + '>icon_from_svg</code> 转换完整的 SVG</li>',
      s2Title: '2. 在 JavaScript / Web 应用中通过 npm 使用 Lucide',
      s2Sub: '把 <code ' + M + '>lucide-static</code> 或 Lucide CDN 与 MorpheusIcons WebAssembly 集成',
      s2aTitle: '方式 A：使用 npm 包 <code ' + T + '>lucide-static</code>',
      s2aIntro: '在你的 Web 项目（Vite、Next.js、Webpack 等）中安装官方包 <code ' + C + '>lucide-static</code>：',
      s2aThen: '然后导入 Lucide 的图标路径，直接传给 MorpheusIcons：',
      s2bTitle: '方式 B：从 CDN 动态获取 Lucide 图标（unpkg / jsDelivr）',
      s2bIntro: '无需任何构建工具！你可以从 Lucide 的 CDN 动态获取任意图标：',
      s3Title: 'GUI 框架集成示例',
      s3Sub: '可直接复制的代码片段，覆盖 GPUI、egui、Iced、Leptos、Dioxus 与 WASM',
      fwGpui: 'GPUI（Zed 编辑器框架）',
      fwEgui: 'egui 立即模式界面',
      fwLeptos: 'Leptos 响应式 Web 组件'
    },

    ja: {
      badge: '好きなアイコンライブラリを — icondata、Lucide、Heroicons、Tabler、Phosphor',
      heroTitle: 'あらゆるプロジェクトで<span class="text-green-700">Cargo・npm・lucide.dev 経由の Lucide</span> を使う',
      heroDesc: 'MorpheusIcons は <strong>Cargo クレート、npm、lucide.dev のいずれから来たアイコンライブラリでも</strong> 動作します。Rust クレートの <code ' + C + '>icondata</code> から始めても、<code ' + C + '>lucide-static</code> のような npm パッケージから SVG を読み込んでも、CDN から取得しても構いません。',
      jumpLabel: '移動:',
      jump1: 'Rust の icondata クレート',
      jump4: 'GUI フレームワーク',
      s1Title: '1. Cargo 経由で Rust から Lucide を使う（<code ' + M + '>icondata</code>）',
      s1Sub: 'Rust クレート <code ' + M + '>icondata</code>、または <code ' + M + '>lucide.dev</code> のサイト SVG を使う',
      s1aTitle: '方法 A: <code ' + T + '>icondata</code> クレートを使う（Rust 向けの Lucide、Heroicons、Tabler）',
      s1aIntro: 'Rust では、広く使われている <code ' + K + '>icondata</code> クレートが Lucide・Heroicons・Tabler・Phosphor のアイコンを Rust の定数としてまとめています:',
      s1aPass: '<code ' + C + '>icondata</code> のパスデータをそのまま MorpheusIcons に渡します:',
      s1bTitle: '方法 B: <code ' + T + '>lucide.dev</code> から直接コピーする',
      s1bSteps: '<li>' + LINK + ' を開きます</li><li>任意のアイコンを検索します（例: <code ' + C + '>play</code>、<code ' + C + '>pause</code>、<code ' + C + '>sun</code>）</li><li><strong>Copy SVG</strong> をクリックするか、<code ' + C + '>.svg</code> ファイルをプロジェクトに保存します</li><li><code ' + K + '>icon_from_svg</code> に渡して SVG 全体を変換します</li>',
      s2Title: '2. JavaScript / Web アプリで npm から Lucide を使う',
      s2Sub: '<code ' + M + '>lucide-static</code> または Lucide CDN を MorpheusIcons WebAssembly と組み合わせる',
      s2aTitle: '方法 A: npm パッケージ <code ' + T + '>lucide-static</code> を使う',
      s2aIntro: 'Web プロジェクト（Vite、Next.js、Webpack など）に公式パッケージ <code ' + C + '>lucide-static</code> をインストールします:',
      s2aThen: '次に Lucide のアイコンパスを import して、そのまま MorpheusIcons に渡します:',
      s2bTitle: '方法 B: CDN から Lucide のアイコンを動的に取得する（unpkg / jsDelivr）',
      s2bIntro: 'ビルドツールは不要です。Lucide の CDN から任意のアイコンを動的に取得できます:',
      s3Title: 'GUI フレームワーク統合の例',
      s3Sub: 'GPUI、egui、Iced、Leptos、Dioxus、WASM 向けのコピーしてすぐ使えるコード',
      fwGpui: 'GPUI（Zed エディタのフレームワーク）',
      fwEgui: 'egui イミディエイトモード UI',
      fwLeptos: 'Leptos リアクティブ Web コンポーネント'
    },

    ko: {
      badge: '원하는 아이콘을 그대로 — icondata, Lucide, Heroicons, Tabler, Phosphor',
      heroTitle: '모든 프로젝트에서 <span class="text-green-700">Cargo, npm, lucide.dev로 Lucide</span> 사용하기',
      heroDesc: 'MorpheusIcons는 <strong>Cargo 크레이트, npm, lucide.dev 어디에서 온 아이콘 라이브러리든</strong> 함께 동작합니다. Rust 크레이트 <code ' + C + '>icondata</code>로 시작하거나, <code ' + C + '>lucide-static</code> 같은 npm 패키지에서 SVG를 불러오거나, CDN에서 가져오면 됩니다.',
      jumpLabel: '이동:',
      jump1: 'Rust icondata 크레이트',
      jump4: 'GUI 프레임워크',
      s1Title: '1. Cargo로 Rust에서 Lucide 사용하기 (<code ' + M + '>icondata</code>)',
      s1Sub: 'Rust 크레이트 <code ' + M + '>icondata</code> 또는 <code ' + M + '>lucide.dev</code> 사이트의 SVG 사용',
      s1aTitle: '방법 A: <code ' + T + '>icondata</code> 크레이트 사용 (Rust용 Lucide, Heroicons, Tabler)',
      s1aIntro: 'Rust에서 널리 쓰이는 <code ' + K + '>icondata</code> 크레이트는 Lucide, Heroicons, Tabler, Phosphor 아이콘을 Rust 상수로 묶어 제공합니다:',
      s1aPass: '<code ' + C + '>icondata</code>의 path 데이터를 MorpheusIcons에 바로 전달하세요:',
      s1bTitle: '방법 B: <code ' + T + '>lucide.dev</code>에서 직접 복사하기',
      s1bSteps: '<li>' + LINK + ' 열기</li><li>원하는 아이콘 검색 (예: <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li><strong>Copy SVG</strong>를 누르거나 <code ' + C + '>.svg</code> 파일을 프로젝트에 저장</li><li><code ' + K + '>icon_from_svg</code>에 전달해 전체 SVG 변환</li>',
      s2Title: '2. JavaScript / 웹 앱에서 npm으로 Lucide 사용하기',
      s2Sub: '<code ' + M + '>lucide-static</code> 또는 Lucide CDN을 MorpheusIcons WebAssembly와 연동',
      s2aTitle: '방법 A: npm 패키지 <code ' + T + '>lucide-static</code> 사용',
      s2aIntro: '웹 프로젝트(Vite, Next.js, Webpack 등)에 공식 패키지 <code ' + C + '>lucide-static</code>을 설치하세요:',
      s2aThen: '그런 다음 Lucide 아이콘 path를 import해 MorpheusIcons에 바로 전달합니다:',
      s2bTitle: '방법 B: CDN에서 Lucide 아이콘 동적으로 가져오기 (unpkg / jsDelivr)',
      s2bIntro: '빌드 도구가 전혀 필요 없습니다. Lucide CDN에서 어떤 아이콘이든 동적으로 가져올 수 있습니다:',
      s3Title: 'GUI 프레임워크 연동 예제',
      s3Sub: 'GPUI, egui, Iced, Leptos, Dioxus, WASM용 바로 복사해 쓰는 코드',
      fwGpui: 'GPUI (Zed 에디터 프레임워크)',
      fwEgui: 'egui 이미디어트 모드 UI',
      fwLeptos: 'Leptos 반응형 웹 컴포넌트'
    },

    ru: {
      badge: 'Любые иконки на ваш выбор — icondata, Lucide, Heroicons, Tabler и Phosphor',
      heroTitle: '<span class="text-green-700">Lucide через Cargo, npm или lucide.dev</span> в любом проекте',
      heroDesc: 'MorpheusIcons работает с <strong>любой библиотекой иконок из крейтов Cargo, npm или lucide.dev</strong>. Начните с Rust-крейта <code ' + C + '>icondata</code>, либо загружайте SVG из npm-пакетов вроде <code ' + C + '>lucide-static</code> или получайте их через CDN.',
      jumpLabel: 'Перейти к:',
      jump1: 'Крейт icondata (Rust)',
      jump4: 'GUI-фреймворки',
      s1Title: '1. Lucide в Rust через Cargo (<code ' + M + '>icondata</code>)',
      s1Sub: 'С помощью Rust-крейта <code ' + M + '>icondata</code> или SVG с сайта <code ' + M + '>lucide.dev</code>',
      s1aTitle: 'Вариант A: крейт <code ' + T + '>icondata</code> (Lucide, Heroicons, Tabler для Rust)',
      s1aIntro: 'В Rust популярный крейт <code ' + K + '>icondata</code> собирает иконки Lucide, Heroicons, Tabler и Phosphor в виде констант Rust:',
      s1aPass: 'Передайте данные пути из <code ' + C + '>icondata</code> напрямую в MorpheusIcons:',
      s1bTitle: 'Вариант B: копирование напрямую с <code ' + T + '>lucide.dev</code>',
      s1bSteps: '<li>Откройте ' + LINK + '</li><li>Найдите любую иконку (например, <code ' + C + '>play</code>, <code ' + C + '>pause</code>, <code ' + C + '>sun</code>)</li><li>Нажмите <strong>Copy SVG</strong> или сохраните файл <code ' + C + '>.svg</code> в проект</li><li>Передайте его в <code ' + K + '>icon_from_svg</code>, чтобы преобразовать SVG целиком</li>',
      s2Title: '2. Lucide из npm в JavaScript / веб-приложениях',
      s2Sub: 'Подключение <code ' + M + '>lucide-static</code> или CDN Lucide к MorpheusIcons WebAssembly',
      s2aTitle: 'Вариант A: npm-пакет <code ' + T + '>lucide-static</code>',
      s2aIntro: 'Установите официальный пакет <code ' + C + '>lucide-static</code> в свой веб-проект (Vite, Next.js, Webpack и т. д.):',
      s2aThen: 'Затем импортируйте пути иконок Lucide и передайте их напрямую в MorpheusIcons:',
      s2bTitle: 'Вариант B: динамическая загрузка иконок Lucide через CDN (unpkg / jsDelivr)',
      s2bIntro: 'Никаких сборщиков не нужно! Любую иконку можно получить динамически через CDN Lucide:',
      s3Title: 'Примеры интеграции с GUI-фреймворками',
      s3Sub: 'Готовые к копированию фрагменты кода для GPUI, egui, Iced, Leptos, Dioxus и WASM',
      fwGpui: 'GPUI (фреймворк редактора Zed)',
      fwEgui: 'egui, интерфейс в immediate-mode',
      fwLeptos: 'Leptos, реактивные веб-компоненты'
    }
  };

  function apply(lang) {
    var dict = GUIDE[lang] || GUIDE.en;
    var nodes = document.querySelectorAll('[data-guide]');
    for (var i = 0; i < nodes.length; i++) {
      var key = nodes[i].dataset.guide;
      var value = dict[key] !== undefined ? dict[key] : GUIDE.en[key];
      if (value !== undefined) nodes[i].innerHTML = value;
    }
  }

  global.MorphGuide = { GUIDE: GUIDE, apply: apply };

  function boot() {
    apply(global.MorphLang ? global.MorphLang.get() : 'en');
  }

  document.addEventListener('morphlang:change', function (e) { apply(e.detail.lang); });

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', boot);
  } else {
    boot();
  }
})(window);
