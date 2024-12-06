// @ts-check

import eslint from "@eslint/js";
import parser from "@typescript-eslint/parser";
import eslintPluginSvelte from "eslint-plugin-svelte";
import globals from "globals";
import tseslint from "typescript-eslint";
import stylisticJs from "@stylistic/eslint-plugin-js";
import stylisticTs from "@stylistic/eslint-plugin-ts";

export default [
	eslint.configs.recommended,
	...tseslint.configs.recommended,
	...eslintPluginSvelte.configs["flat/recommended"], // Use the recommended Svelte config
	{
		languageOptions: {
			globals: {
				...globals.node,
				...globals.browser,
			},
			parserOptions: {
				parser: parser,
			},
		},
		plugins: {
			"@stylistic/js": stylisticJs,
			"@stylistic/ts": stylisticTs,
		},
		rules: {
			"@stylistic/ts/indent": ["error", "tab"],
			"@stylistic/ts/semi": "error",
			"@stylistic/ts/quotes": ["error", "double"],
			"@stylistic/ts/space-before-blocks": "error",
			"@stylistic/ts/quote-props": ["error", "as-needed"],
			"@stylistic/js/no-multi-spaces": "error",

			"@typescript-eslint/explicit-function-return-type": "error",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^(_|\\$\\$)",
				},
			],
			"svelte/block-lang": [
				"error",
				{
					enforceScriptPresent: true,
					script: ["ts"],
				},
			],
			"svelte/indent": [
				"error",
				{
					indent: "tab",
				},
			],
		},
	},
	{
		ignores: ["node_modules/**", ".svelte-kit/**", "src-tauri", "build"],
	},
];
