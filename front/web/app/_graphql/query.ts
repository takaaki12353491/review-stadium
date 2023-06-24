import { gql } from "@apollo/client";

export const QUERY = gql`
  type Query {
    users: [User]
  }
`;

export const GET_USERS = gql`
  query GetUsers {
    users {
      id
      name
      created_at
    }
  }
`;
