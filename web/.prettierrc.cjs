/** @type { import("prettier").Config } */
module.exports = {
	arrowParens: "always",
	bracketSameLine: true,
	bracketSpacing: true,
	endOfLine: "auto",
	overrides: [
		{
			files: "*.html",
			options: {
				parser: "angular",
			},
		},
	],
	printWidth: 120,
	semi: true,
	singleQuote: false,
	tabWidth: 4,
	trailingComma: "all",
	useTabs: true,
};
