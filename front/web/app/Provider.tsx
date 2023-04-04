"use client";

import { Amplify } from "aws-amplify";
import awsExports from "@/app/aws-exports";
import { ChakraProvider } from "@chakra-ui/react";

Amplify.configure({ ...awsExports, ssr: true });

export default function Provider({ children }: { children: React.ReactNode }) {
  return <ChakraProvider>{children}</ChakraProvider>;
}
