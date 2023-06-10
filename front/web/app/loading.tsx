import { Spinner, Flex } from "@/app/_components/chakra-ui";

export default function Loading() {
  return (
    <Flex
      width={"100%"}
      height={"100%"}
      justifyContent={"center"}
      alignItems={"center"}
      marginTop={"200px"}
    >
      <Spinner />
    </Flex>
  );
}
