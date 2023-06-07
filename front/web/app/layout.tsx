import "./_styles/globals.scss";
import "@aws-amplify/ui-react/styles.css";
import Provider from "./Provider";
import { Container, Box } from "@/app/_components/chakra-ui";
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
          <Box as={"main"} bg={"bg.primary"} py={"10"}>
            <Container
              as={"div"}
              bg={"white"}
              py={"4"}
              maxW={"container.xl"}
              minH={"calc(100vh - 115px - 2rem)"}
            >
              {children}
            </Container>
          </Box>
          <Footer />
        </Provider>
      </body>
    </html>
  );
}
