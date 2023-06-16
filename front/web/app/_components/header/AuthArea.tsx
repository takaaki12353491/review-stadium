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
    <Menu>
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
        <MenuItem>Link 1</MenuItem>
        <MenuItem>Link 2</MenuItem>
        <MenuDivider />
        <MenuItem onClick={() => signOut()}>ログアウト</MenuItem>
      </MenuList>
    </Menu>
  );
}

export default function AuthArea() {
  const { data: session, status } = useSession();

  if (status === "loading") return null;

  const user = session?.user;

  // Render buttons based on authentication status
  return (
    <Flex alignItems={"center"}>
      {user ? (
        <AuthenticatedArea image={user.image || undefined} />
      ) : (
        <UnauthenticatedArea />
      )}
    </Flex>
  );
}
