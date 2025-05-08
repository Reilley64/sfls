// Learn more https://docs.expo.io/guides/customizing-metro
const { getDefaultConfig } = require("expo/metro-config");
const { withNativeWind } = require("nativewind/metro");
const path = require("path");

/** @type {import('expo/metro-config').MetroConfig} */
const projectRoot = __dirname;
const monorepoRoot = path.resolve(projectRoot, "../..");
const config = getDefaultConfig(__dirname); // eslint-disable-line no-undef

config.watchFolders = [monorepoRoot];
config.resolver.nodeModulesPaths = [
  path.resolve(projectRoot, "node_modules"),
  path.resolve(monorepoRoot, "node_modules"),
];
if (process.env?.EXPO_TV === "1") {
  const originalSourceExts = config.resolver.sourceExts;
  config.resolver.sourceExts = [...originalSourceExts.map((e) => `tv.${e}`), ...originalSourceExts];
}

module.exports = withNativeWind(config, { input: "./global.css" });
