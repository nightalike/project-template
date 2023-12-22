// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

import { Config } from "jest";

const config: Config = {
  roots: ["<rootDir>/src"],
  transform: {
    "^.+\\.(ts|tsx)$": "ts-jest"
  }
};

export default config;
