"use client";

import {
  Flex,
  Avatar,
  Button,
  Menu,
  MenuButton,
  MenuList,
  MenuItem,
  MenuDivider,
} from "@chakra-ui/react";
import NextLink from "next/link";
import { signIn, signOut, useSession } from "next-auth/react";

const UnauthenticatedArea = () => {
  return (
    <Button
      onClick={() => signIn()}
      fontSize="sm"
      fontWeight={600}
      color="white"
      bg="orange.400"
      _hover={{
        bg: "orange.300",
      }}
    >
      ログイン
    </Button>
  );
};

function AuthenticatedArea({ image }: { image?: string }) {
  return (
    <Menu placement="bottom-end">
      <MenuButton
        as={Button}
        rounded={"full"}
        variant={"link"}
        cursor={"pointer"}
        minW={0}
      >
        <Avatar size={"sm"} src={image} />
      </MenuButton>
      <MenuList>
        <MenuItem as={NextLink} href={"/settings"}>
          設定
        </MenuItem>
        <MenuDivider />
        <MenuItem onClick={() => signOut()}>ログアウト</MenuItem>
      </MenuList>
    </Menu>
  );
}

export default function AuthArea() {
  const { data: session, status } = useSession();

  let content;
  if (status === "loading") {
    content = null;
  } else {
    const user = session?.user;
    content = user ? (
      <AuthenticatedArea image={user.image || undefined} />
    ) : (
      <UnauthenticatedArea />
    );
  }

  return (
    <Flex w={20} alignItems={"center"}>
      {content}
    </Flex>
  );
}
