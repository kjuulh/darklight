import {useMemo} from 'react';
import merge from 'deepmerge';
import type {GetServerSidePropsContext} from 'next';
import type {NormalizedCacheObject} from '@apollo/client';
import {ApolloClient, HttpLink, InMemoryCache, split} from '@apollo/client';
import isEqual from 'lodash.isequal';
import {GraphQLWsLink} from "@apollo/client/link/subscriptions";
import {createClient} from "graphql-ws";
import {getMainDefinition} from "@apollo/client/utilities";

interface PageProps {
    props?: Record<string, any>;
}

export const APOLLO_STATE_PROPERTY_NAME = '__APOLLO_STATE__';

let apolloClient: ApolloClient<NormalizedCacheObject> | null = null;

const createApolloClient = (ctx?: GetServerSidePropsContext) => {
    const httpLink = new HttpLink({
        uri: process.env.NEXT_PUBLIC_GRAPHQL_URI,
        credentials: 'same-origin',
    });

    const graphqlWsUri = process.env.NEXT_PUBLIC_GRAPHQL_WS_URI;
    if (!graphqlWsUri) {
        throw new Error("could not find ws url")
    }

    const wsLink =
        typeof window !== "undefined"
            ? new GraphQLWsLink(
                createClient({
                    url: graphqlWsUri,
                })
            )
            : null;

    const link =
        typeof window !== "undefined" && wsLink != null
            ? split(
                ({query}) => {
                    const def = getMainDefinition(query);
                    return (
                        def.kind === "OperationDefinition" &&
                        def.operation === "subscription"
                    );
                },
                wsLink,
                httpLink
            )
            : httpLink;

    return new ApolloClient({
        ssrMode: typeof window === 'undefined',
        link: link,
        cache: new InMemoryCache(),
    });
};

export function initializeApollo(initialState = null, ctx = null) {
    // @ts-ignore
    const client = apolloClient ?? createApolloClient(ctx);

    // If your page has Next.js data fetching methods that use Apollo Client,
    // the initial state gets hydrated here
    if (initialState) {
        // Get existing cache, loaded during client side data fetching
        const existingCache = client.extract();

        // Merge the existing cache into data passed from getStaticProps/getServerSideProps
        const data = merge(initialState, existingCache, {
            // combine arrays using object equality (like in sets)
            arrayMerge: (destinationArray, sourceArray) => [
                ...sourceArray,
                ...destinationArray.filter((d) =>
                    sourceArray.every((s) => !isEqual(d, s)),
                ),
            ],
        });

        // Restore the cache with the merged data
        client.cache.restore(data);
    }

    // For SSG and SSR always create a new Apollo Client
    if (typeof window === 'undefined') {
        return client;
    }

    // Create the Apollo Client once in the client
    if (!apolloClient) {
        apolloClient = client;
    }

    return client;
}

export function addApolloState(
    client: ApolloClient<NormalizedCacheObject>,
    pageProps: PageProps,
) {
    if (pageProps?.props) {
        pageProps.props[APOLLO_STATE_PROPERTY_NAME] = client.cache.extract();
    }

    return pageProps;
}

export function useApollo(pageProps: PageProps) {
    // @ts-ignore
    const state = pageProps[APOLLO_STATE_PROPERTY_NAME];
    const store = useMemo(() => initializeApollo(state), [state]);

    return store;
}