import type { CodegenConfig } from "@graphql-codegen/cli";

const config: CodegenConfig = {
  overwrite: true,
  schema: "app/_graphql/**/*.ts",
  documents: "app/_graphql/**/*.ts",
  generates: {
    "app/_types/graphql/": {
      preset: "client",
      plugins: [
        "typescript",
        "typescript-operations",
        "typescript-graphql-request",
      ],
    },
  },
};

export default config;
