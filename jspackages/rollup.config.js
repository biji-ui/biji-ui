import resolve from "@rollup/plugin-node-resolve";
import terser from "@rollup/plugin-terser";

export default {
  input: "scripts/shiki.js",
  output: {
    file: "bundles/shiki.js",
    format: "es",
    plugins: [
      terser({
        compress: {
          passes: 2,
          drop_console: true,
          pure_getters: true,
        },
        mangle: true,
      }),
    ],
  },
  plugins: [
    resolve({
      browser: true,
      exportConditions: ["browser"],
    }),
  ],
};
