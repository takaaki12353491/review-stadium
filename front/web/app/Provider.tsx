"use client";

import { ChakraProvider } from "@chakra-ui/react";
import { theme } from "@/app/_styles/theme";

export default function Provider({ children }: { children: React.ReactNode }) {
  return <ChakraProvider theme={theme}>{children}</ChakraProvider>;
}
