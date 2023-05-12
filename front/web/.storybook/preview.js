import * as nextImage from "next/image";
import { RouterContext } from "next/dist/shared/lib/router-context";
import "../app/styles/globals.scss";

Object.defineProperty(nextImage, "default", {
  configurable: true,
  value: (props) => {
    return <img {...props} />;
  },
});

/** @type { import('@storybook/react').Preview } */
const preview = {
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    nextRouter: {
      Provider: RouterContext.Provider,
    },
  },
};

export default preview;
