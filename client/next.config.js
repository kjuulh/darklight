const withPWA = require("next-pwa");
const runtimeCaching = require("next-pwa/cache");
const withBundleAnalyzer = require('@next/bundle-analyzer')({
    enabled: process.env.ANALYZE === 'true'
})

const withPlugins = require('next-compose-plugins')

module.exports = withPlugins([
    withPWA({
        pwa: {
            disable: process.env.NODE_ENV === "development",
            dest: "public",
            runtimeCaching,
        },
        experimental: {
            outputStandalone: true,
        },
        webpack(config, options) {
            config.module.rules.push({
                test: /\.ya?ml$/,
                type: 'json',
                use: 'yaml-loader',
            })

            return config
        },
    }),
    withBundleAnalyzer

]);


