import hljs from "highlight.js";
import rust from "highlight.js/lib/languages/rust";
import bash from "highlight.js/lib/languages/bash";

import "highlight.js/styles/a11y-dark.css";

hljs.registerLanguage("rust", rust);
hljs.registerLanguage("bash", bash);

export function highlight(code, language) {
  return hljs.highlight(code, { language }).value;
}

export function highlightElement(block) {
  hljs.highlightElement(block);
}
