import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

let publicApiUrl = process.env.PUBLIC_API_URL;

if (publicApiUrl && !publicApiUrl.endsWith("/")) {
    publicApiUrl += "/";
}

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({
            pages: "build",
            assets: "build",
            fallback: undefined,
            precompress: false,
            strict: true
        }),
        csp: {
            mode: "hash", // For prerendered mode
            directives: {
                "base-uri": ["none"],
                "connect-src": publicApiUrl ? ["self", publicApiUrl] : ["self"],
                "default-src": ["self"],
                "frame-src": ["none"],
                "object-src": ["none"],
                "script-src": ["self"],
                "style-src": ["self", "unsafe-inline"]
            }
        }
    }
};

export default config;
