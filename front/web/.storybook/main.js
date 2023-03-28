const path = require("path");
const rootPath = path.resolve(__dirname, "../app/");

/** @type { import('@storybook/react-webpack5').StorybookConfig } */
const config = {
  stories: ["../app/**/*.mdx", "../app/**/*.stories.@(js|jsx|ts|tsx)"],
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-interactions",
    // "storybook-addon-next-router",
    "@storybook/addon-a11y",
    {
      name: "@storybook/addon-postcss",
      options: {
        postcssLoaderOptions: {
          implementation: require("postcss"),
        },
      },
    },
  ],
  framework: {
    name: "@storybook/react-webpack5",
    options: {},
  },
  docs: {
    autodocs: "tag",
  },
  staticDirs: ["../public"],
  webpackFinal: async (config) => {
    config.resolve.alias["@"] = rootPath;
    config.module.rules.push({
      test: /\.scss$/,
      use: [
        "style-loader",
        {
          loader: "css-loader",
          options: {
            modules: {
              auto: true, // *.module.scssファイル全てを対象
            },
            url: false, // cssのbackgroundで設定した画像へのパスがプロジェクトルートからの絶対パスになるように設定
          },
        },
        "sass-loader",
      ],
      include: path.resolve(__dirname, "../app/"),
    });
    return config;
  },
};
export default config;
