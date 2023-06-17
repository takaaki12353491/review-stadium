"use client";

import {
  Box,
  Flex,
  HStack,
  useColorModeValue,
} from "@/app/_components/chakra-ui";
import AuthArea from "./AuthArea";
import NextLink from "next/link";

export default function Header() {
  return (
    <Box
      as={"header"}
      position="sticky"
      top={0}
      zIndex={1000}
      boxShadow={"lg"}
      px={4}
      bg={useColorModeValue("white", "gray.800")}
    >
      <Flex h={16} alignItems={"center"} justifyContent={"space-around"}>
        <HStack spacing={8} alignItems={"center"}>
          <NextLink href="/">Logo</NextLink>
        </HStack>
        <AuthArea />
      </Flex>
    </Box>
  );
}
