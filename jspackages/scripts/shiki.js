import { createHighlighterCore } from "shiki/core";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";

import light from "@shikijs/themes/solarized-light";
import dark from "@shikijs/themes/vesper";

import rust from "@shikijs/langs/rust";
import bash from "@shikijs/langs/bash";

import wasm from "shiki/wasm";

const highlighter = await createHighlighterCore({
  themes: [light, dark],
  langs: [rust, bash],
  engine: createOnigurumaEngine(wasm),
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
