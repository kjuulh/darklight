import Head from "next/head";
import "../styles/globals.css";
import {AppProps} from "next/app";
import {ApolloProvider} from '@apollo/client';

import {useApollo} from '../lib/apolloClient';

export default function MyApp({Component, pageProps}: AppProps) {
    const apolloClient = useApollo(pageProps);

    return (
        <>
            <Head>
                <meta charSet="utf-8"/>
                <meta httpEquiv="X-UA-Compatible" content="IE=edge"/>
                <meta
                    name="viewport"
                    content="width=device-width,initial-scale=1,minimum-scale=1,maximum-scale=1,user-scalable=no"
                />
                <meta name="description" content="DarkLight, download files"/>
                <meta name="keywords" content="file downloader darklight"/>
                <title>DarkLight - Download Files</title>

                <link rel="manifest" href="/manifest.json"/>
                <meta name="theme-color" content="#317EFB"/>
                <link
                    href="/android/android-launchericon-48-48.png"
                    rel="icon"
                    type="image/png"
                    sizes="48x48"
                />
            </Head>
            <ApolloProvider client={apolloClient}>
                <Component {...pageProps} />
            </ApolloProvider>
        </>
    );
}
