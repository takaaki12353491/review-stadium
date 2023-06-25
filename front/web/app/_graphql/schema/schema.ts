import { gql } from "@apollo/client";

export const SCHEMA = gql`
  schema {
    query: Query
  }
`;

export const QUERY = gql`
  type Query {
    users: [User]
  }
`;

export const USER = gql`
  type User {
    id: ID!
    name: String
    email: String
    created_at: String
  }
`;
