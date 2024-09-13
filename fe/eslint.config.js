import globals from "globals";
import jsEslint from "@eslint/js";
import tsEslint from "typescript-eslint";
import svelteEslint from "eslint-plugin-svelte";
import svelteParser from "svelte-eslint-parser";
import eslintConfigPrettier from "eslint-config-prettier";

const ignores = [
    "eslint.config.js",
    "svelte.config.js",
    "vite.config.ts",
    ".svelte-kit",
    "build",
    "node_modules",
    "static"
];

/** @type {import('eslint').Linter.Config[]} */
export default [
    { ignores },
    jsEslint.configs.recommended,
    ...tsEslint.configs.strictTypeChecked,
    ...tsEslint.configs.stylisticTypeChecked,
    ...svelteEslint.configs["flat/recommended"],
    eslintConfigPrettier,
    ...svelteEslint.configs["flat/prettier"],
    {
        languageOptions: {
            parserOptions: {
                parser: tsEslint.parser,
                projectService: true,
                tsconfigRootDir: import.meta.dirname,
                extraFileExtensions: [".svelte"]
            }
        }
    },
    {
        rules: {
            "no-console": ["error", { allow: ["error"] }],
            "no-else-return": "error"
        }
    },
    {
        files: ["**/*.svelte"],
        languageOptions: {
            parser: svelteParser,
            parserOptions: {
                // Select which parser to use for the script tags
                parser: tsEslint.parser,
                extraFileExtensions: [".svelte"]
            },
            globals: {
                ...globals.browser
            }
        },
        rules: {
            "svelte/sort-attributes": "error",
            "@typescript-eslint/no-confusing-void-expression": "off"
        }
    }
];
