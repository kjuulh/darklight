const withPWA = require("next-pwa");
const runtimeCaching = require("next-pwa/cache");

module.exports = withPWA({
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
});