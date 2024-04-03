const esbuild = require("esbuild");
esbuild
  .build({
    entryPoints: ["./deps/highlight.js"],
    bundle: true,
    outfile: "./output/highlight.js",
    format: "esm",
    minify: true,
  })
  .catch(() => process.exit(1));
