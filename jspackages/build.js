const esbuild = require("esbuild");

esbuild
  .build({
    entryPoints: ["./deps/shiki.js"],
    bundle: true,
    outfile: "./output/shiki.js",
    format: "esm",
    minify: true,
  })
  .catch(() => process.exit(1));
