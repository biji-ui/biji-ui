import { getHighlighterCore } from "shiki/core";

import getWasm from "shiki/wasm";

import solarizedLight from "shiki/themes/solarized-light.mjs";
import vesper from "shiki/themes/vesper.mjs";

const highlighter = await getHighlighterCore({
  themes: [solarizedLight, vesper],
  langs: [import("shiki/langs/rust.mjs"), import("shiki/langs/bash.mjs")],
  loadWasm: getWasm,
});

/**
 * Syntax highlight code in HTML
 * @param {string} code
 * @param {string} lang
 * @param {string} theme
 * @returns {string}
 */
export function codeToHtml(code, lang, theme) {
  const converted = highlighter.codeToHtml(code, {
    lang,
    theme,
  });

  return converted;
}
