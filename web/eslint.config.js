import angular from "@angular-eslint/eslint-plugin";
import angularTemplate from "@angular-eslint/eslint-plugin-template";
import angularTemplateParser from "@angular-eslint/template-parser";
import eslintTypescript from "@typescript-eslint/eslint-plugin";
import typescriptParser from "@typescript-eslint/parser";
import { defineFlatConfig } from "eslint-define-config";
import eslintImport from "eslint-plugin-import";
import jsdoc from "eslint-plugin-jsdoc";
import n from "eslint-plugin-n";
import nodeImport from "eslint-plugin-node-import";
import perfectionist from "eslint-plugin-perfectionist";
import preferArrow from "eslint-plugin-prefer-arrow";
import preferLet from "eslint-plugin-prefer-let";
import promise from "eslint-plugin-promise";
import sonarjs from "eslint-plugin-sonarjs";
import unicorn from "eslint-plugin-unicorn";
import globals from "globals";

import { importRules } from "./tool/eslint/rules/import.js";
import { nRules } from "./tool/eslint/rules/n.js";
import { promiseRules } from "./tool/eslint/rules/promise.js";
import { sonarjsRules } from "./tool/eslint/rules/sonarjs.js";
import { styleRules } from "./tool/eslint/rules/style.js";
import { typescriptRules } from "./tool/eslint/rules/typescript.js";
import { unicornRules } from "./tool/eslint/rules/unicorn.js";

// eslint-disable-next-line import/no-default-export
export default defineFlatConfig([
	{
		ignores: ["**/node_modules/**", "**/dist/**", "**/build/**", ".git/**"],
	},
	perfectionist.configs["recommended-alphabetical"],
	{
		files: ["**/*.js", "**/*.ts", "**/*.d.ts"],
		languageOptions: {
			ecmaVersion: 2023,
			globals: {
				...globals.browser,
				...globals.es2021,
				...globals.node,
			},
			sourceType: "module",
		},
		plugins: {
			import: eslintImport,
			jsdoc,
			n,
			"node-import": nodeImport,
			"prefer-arrow": preferArrow,
			"prefer-let": preferLet,
			promise,
			sonarjs,
			unicorn,
		},
		rules: {
			...unicornRules,
			...sonarjsRules,
			...importRules,
			...promiseRules,
			...nRules,
			...styleRules,
			"arrow-body-style": "off",
			"perfectionist/sort-imports": [
				"error",
				{
					customGroups: {
						type: {
							angular: ["@angular*"],
							charts: ["chart.js*", "chartjs*"],
							luxon: ["luxon"],
							ramda: ["ramda", "ramda-adjunct"],
						},
						value: {
							angular: ["@angular*"],
							charts: ["chart.js*", "chartjs*"],
							luxon: ["luxon"],
							ramda: ["ramda", "ramda-adjunct"],
						},
					},
					groups: [
						"angular",
						"luxon",
						"ramda",
						"charts",
						"type",
						["builtin", "external"],
						"internal-type",
						"internal",
						["parent-type", "sibling-type", "index-type"],
						["parent", "sibling", "index"],
						"object",
						"unknown",
						"style",
					],
					newlinesBetween: "always",
					order: "asc",
					type: "alphabetical",
				},
			],
			"unicorn/filename-case": ["error", { cases: { kebabCase: true } }],
			"unicorn/prevent-abbreviations": ["error", { ignore: ["env", "Env"] }],
		},
	},
	{
		files: ["**/*.ts", "**/*.d.ts"],
		languageOptions: {
			parser: typescriptParser,
			parserOptions: {
				projectService: true,
				tsconfigRootDir: import.meta.dirname,
			},
		},
		plugins: {
			"@angular-eslint": angular,
			"@typescript-eslint": eslintTypescript,
			jsdoc,
		},
		rules: {
			...typescriptRules,
			...angular.configs.recommended.rules,
			...jsdoc.configs["flat/recommended-typescript"].rules,
		},
		settings: {
			"import/resolver": {
				typescript: {
					project: ["./tsconfig.json"],
				},
			},
		},
	},
	{
		files: ["**/*.html"],
		languageOptions: {
			parser: angularTemplateParser,
		},
		plugins: {
			"@angular-eslint/template": angularTemplate,
		},
		rules: {
			...angularTemplate.configs.recommended.rules,
			...angularTemplate.configs.accessibility.rules,
		},
	},
	{
		files: ["**/*.component.ts"],
		rules: {
			"import/no-default-export": "off",
			"perfectionist/sort-exports": "off",
			"unicorn/prefer-export-from": "off",
		},
	},
]);
