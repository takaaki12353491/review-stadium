import "./styles/globals.scss";
import "@aws-amplify/ui-react/styles.css";
import Provider from "./Provider";
import { Container } from "@/app/components/chakra-ui";
import Header from "./Header";
import Footer from "./Footer";
import { Amplify } from "aws-amplify";

import awsExports from "./aws-exports";
Amplify.configure(awsExports);

export const metadata = {
  title: "レビュースタジアム",
  description: "サッカーのレビューサイトです。",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <head />
      <body>
        <Provider>
          <Header />
          <Container
            as="main"
            maxW="container.lg"
            my="4"
            minH="calc(100vh - 115px - 2rem)"
          >
            {children}
          </Container>
          <Footer />
        </Provider>
      </body>
    </html>
  );
}
