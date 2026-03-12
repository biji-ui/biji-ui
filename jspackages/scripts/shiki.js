import { createHighlighterCore } from "shiki/core";
import { createOnigurumaEngine } from "shiki/engine/oniguruma";

import light from "@shikijs/themes/solarized-light";
import dark from "@shikijs/themes/vesper";

import rust from "@shikijs/langs/rust";
import bash from "@shikijs/langs/bash";
import toml from "@shikijs/langs/toml";

import wasm from "shiki/wasm";

// Override vesper comment color: #8B8B8B94 (semi-transparent, fails WCAG AA) → #8B8B8B (opaque, ~6:1 on #101010)
const darkAccessible = {
  ...dark,
  tokenColors: dark.tokenColors.map((tc) =>
    Array.isArray(tc.scope) && tc.scope.includes("comment")
      ? { ...tc, settings: { ...tc.settings, foreground: "#8b8b8b" } }
      : tc
  ),
};

const highlighter = await createHighlighterCore({
  themes: [light, darkAccessible],
  langs: [rust, bash, toml],
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
