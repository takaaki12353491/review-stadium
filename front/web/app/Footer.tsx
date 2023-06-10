import { Container, Box, Text } from "@/app/_components/chakra-ui";

export default function Footer() {
  return (
    <Box bg="white" as="footer">
      <Container maxW="5xl" py={4}>
        <Text as="small">© 2023 takaaki12353491</Text>
      </Container>
    </Box>
  );
}
