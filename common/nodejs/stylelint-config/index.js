module.exports = {
  extends: [
    "stylelint-config-standard",
    "stylelint-config-standard-scss",
    "stylelint-config-recess-order",
  ],
  plugins: ["stylelint-order"],
  rules: {
    "scss/at-rule-no-unknown": [
      true,
      {
        ignoreAtRules: ["apply", "layer", "responsive", "screen", "tailwind"],
      },
    ],
  },
};
