/* tslint:disable */
/* eslint-disable */
//  This file was automatically generated and should not be edited.

export type Post = {
  __typename: "Post",
  id: string,
  title: string,
  content: string,
};

export type ListPostsQuery = {
  listPosts?:  Array< {
    __typename: "Post",
    id: string,
    title: string,
    content: string,
  } | null > | null,
};
