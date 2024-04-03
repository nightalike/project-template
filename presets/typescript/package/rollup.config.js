// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

import typescript from "@rollup/plugin-typescript";
import copy from "rollup-plugin-copy";
import pkg from "./package.json" assert { type: "json" };

/** @type {import("rollup").RollupOptions} */
export default {
  input: "src/index.ts",
  output: {
    format: "esm",
    file: "build/index.js",
    sourcemap: true
  },
  plugins: [
    copy({
      targets: [
        { src: "src/types", dest: "build" },
        {
          src: "package.json",
          dest: "build"
        },
        {
          src: "README.md",
          dest: "build"
        },
        {
          src: "LICENSE",
          dest: "build"
        }
      ]
    }),
    typescript({
      tsconfig: "tsconfig.json"
    })
  ],
  external: [...Object.keys(pkg.dependencies || {})],
  onwarn(warning, warn) {
    if (warning.code === "CIRCULAR_DEPENDENCY") {
      return;
    }
    warn(warning);
  }
};
