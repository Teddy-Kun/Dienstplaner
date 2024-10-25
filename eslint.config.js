// @ts-check

import eslint from "@eslint/js";
import parser from "@typescript-eslint/parser";
import biome from "eslint-config-biome";
import eslintPluginSvelte from "eslint-plugin-svelte";
import globals from "globals";
import tseslint from "typescript-eslint";

export default [
	eslint.configs.recommended,
	...tseslint.configs.recommended,
	...eslintPluginSvelte.configs["flat/prettier"], // Use the recommended Svelte config
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
		rules: {
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
				},
			],
			"svelte/block-lang": [
				"error",
				{
					enforceScriptPresent: true,
					script: ["ts"],
					style: "postcss",
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
	biome,
];
