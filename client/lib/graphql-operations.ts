import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type Download = {
  __typename?: 'Download';
  file?: Maybe<Scalars['String']>;
  id: Scalars['ID'];
  link: Scalars['String'];
  percentage: Scalars['Int'];
  state: Scalars['String'];
};

export type DownloadChanged = {
  __typename?: 'DownloadChanged';
  download?: Maybe<Download>;
  id: Scalars['ID'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  requestDownload: RequestDownloadResp;
};


export type MutationRootRequestDownloadArgs = {
  link: Scalars['String'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  getDownload?: Maybe<Download>;
  helloWorld: Scalars['String'];
};


export type QueryRootGetDownloadArgs = {
  downloadId: Scalars['ID'];
};

export type RequestDownloadResp = {
  __typename?: 'RequestDownloadResp';
  id: Scalars['ID'];
};

export type SubscriptionRoot = {
  __typename?: 'SubscriptionRoot';
  getDownload: DownloadChanged;
};


export type SubscriptionRootGetDownloadArgs = {
  downloadId: Scalars['ID'];
};

export type GetDownloadQueryVariables = Exact<{
  downloadId: Scalars['ID'];
}>;


export type GetDownloadQuery = { __typename?: 'QueryRoot', getDownload?: { __typename?: 'Download', id: string, link: string, percentage: number } | null };

export type SubscribeDownloadSubscriptionVariables = Exact<{
  downloadId: Scalars['ID'];
}>;


export type SubscribeDownloadSubscription = { __typename?: 'SubscriptionRoot', getDownload: { __typename?: 'DownloadChanged', download?: { __typename?: 'Download', id: string, link: string, file?: string | null, percentage: number } | null } };


export const GetDownloadDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetDownload"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"downloadId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"getDownload"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"downloadId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"downloadId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"link"}},{"kind":"Field","name":{"kind":"Name","value":"percentage"}}]}}]}}]} as unknown as DocumentNode<GetDownloadQuery, GetDownloadQueryVariables>;
export const SubscribeDownloadDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"SubscribeDownload"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"downloadId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"ID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"getDownload"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"downloadId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"downloadId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"download"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"id"}},{"kind":"Field","name":{"kind":"Name","value":"link"}},{"kind":"Field","name":{"kind":"Name","value":"file"}},{"kind":"Field","name":{"kind":"Name","value":"percentage"}}]}}]}}]}}]} as unknown as DocumentNode<SubscribeDownloadSubscription, SubscribeDownloadSubscriptionVariables>;