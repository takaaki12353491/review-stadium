import { gql } from "@apollo/client";

export const USER = gql`
  type User {
    id: ID!
    name: String
    email: String
    created_at: String
  }
`;
