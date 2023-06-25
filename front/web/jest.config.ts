import type { Config } from "jest";

export default async (): Promise<Config> => {
  return {
    /** @link https://stackoverflow.com/questions/50863312/jest-gives-cannot-find-module-when-importing-components-with-absolute-paths */
    moduleDirectories: ["node_modules", "<rootDir>/"],
    roots: ["<rootDir>/app/"],
    testEnvironment: "jsdom",
    testPathIgnorePatterns: [
      "<rootDir>/node_modules/",
      "<rootDir>/src/__tests__/utils.tsx",
    ],
    moduleNameMapper: {
      "\\.(css|scss)$": "<rootDir>/node_modules/jest-css-modules",
    },
    transform: {
      "^.+\\.(ts|tsx)$": ["babel-jest", { presets: ["next/babel"] }],
    },
  };
};
