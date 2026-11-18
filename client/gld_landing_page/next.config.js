module.exports = {
  distDir: "dist",
  output: "export",
  // Every page is exported as <path>/index.html. The asset canister then
  // serves /docs and /docs/ alike; with <path>.html only the first form works
  // and the second falls back to the home page.
  trailingSlash: true,
};
