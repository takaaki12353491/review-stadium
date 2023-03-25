module.exports = {
  extends: [
    "stylelint-config-standard",
    "stylelint-config-recess-order",
  ],
  plugins: [
    "stylelint-order"
  ],
  rules: {
    'at-rule-no-unknown': null,
  }
}