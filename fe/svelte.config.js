import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

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
            mode: "auto",
            directives: {
                "script-src": ["self"],
                "style-src": ["self", "unsafe-inline"],
                "object-src": ["none"],
                "frame-src": ["none"],
                "base-uri": ["none"]
            }
        }
    }
};

export default config;
