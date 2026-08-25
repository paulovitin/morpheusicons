#!/usr/bin/env node
/**
 * Keeps the no-build fallback <style> block in every page under pages/ in sync with
 * src/input.css, which is the single source of truth for hand-written CSS.
 *
 * The pages ship a Tailwind CDN fallback for when dist/output.css is
 * missing, so it needs a copy of these rules inline. Maintaining that copy
 * by hand is how the two files silently diverged before. Run via
 * `npm run sync:styles` (build:css does it automatically).
 */
import { readFileSync, writeFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

const START = '<!-- impeccable:styles:start — generated from src/input.css, do not edit -->';
const END = '<!-- impeccable:styles:end -->';

const css = readFileSync('src/input.css', 'utf8')
  .split('\n')
  .filter((line) => !line.startsWith('@tailwind') && !line.startsWith('@import "tailwindcss"') && !line.startsWith('@config'))
  .join('\n')
  .trim();

const indented = css
  .split('\n')
  .map((line) => (line.length ? `    ${line}` : ''))
  .join('\n');

const block = `${START}\n  <style>\n${indented}\n  </style>\n  ${END}`;

const PAGES_DIR = 'pages';
const targets = readdirSync(PAGES_DIR)
  .filter((f) => f.endsWith('.html'))
  .map((f) => join(PAGES_DIR, f));
const pattern = new RegExp(
  `${START.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}[\\s\\S]*?${END}`
);

for (const target of targets) {
  const html = readFileSync(target, 'utf8');
  if (!pattern.test(html)) {
    console.error(`sync-styles: markers not found in ${target}. Aborting.`);
    process.exit(1);
  }
  const next = html.replace(pattern, block);
  if (next === html) {
    console.log(`sync-styles: ${target} already up to date.`);
  } else {
    writeFileSync(target, next);
    console.log(`sync-styles: ${target} inline <style> regenerated from src/input.css.`);
  }
}
