// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

/** @type {import("semantic-release").GlobalConfig} */
export default {
  branches: ["main", { name: "beta", prerelease: true }]
};
