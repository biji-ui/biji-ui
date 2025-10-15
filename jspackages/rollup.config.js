import babel from "@rollup/plugin-babel";
import resolve from "@rollup/plugin-node-resolve";
import terser from "@rollup/plugin-terser";

const config = [
  {
    input: "scripts/shiki.js",
    output: [
      {
        // dir: "output",
        file: "bundles/shiki.js",
        format: "es",
        plugins: [terser()],
      },
    ],
    plugins: [
      babel({
        exclude: "node_modules/**",
        presets: [["@babel/preset-env", { targets: "defaults" }]],
        babelHelpers: "bundled",
      }),
      resolve({
        browser: true,
      }),
      // commonjs(),
    ],
  },
];

export default config;
