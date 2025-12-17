module.exports = {
	extends: ["stylelint-config-recommended-scss", "stylelint-config-recess-order"],
	ignoreFiles: ["**/*.js", "**/*.html", "dist/**/*", "assets/**/*", "build/**/*"],
	plugins: ["stylelint-scss"],
	rules: {
		"alpha-value-notation": "percentage",
		"annotation-no-unknown": true,
		"at-rule-empty-line-before": null,
		"at-rule-no-unknown": null,
		"at-rule-no-vendor-prefix": true,
		"block-no-empty": true,
		"color-function-notation": "legacy",
		"comment-empty-line-before": [
			"always",
			{
				except: ["first-nested"],
				ignore: ["stylelint-commands"],
			},
		],
		"comment-no-empty": true,
		"comment-whitespace-inside": "always",
		"custom-media-pattern": [
			"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$",
			{
				message: (value) => `Expected custom media query value "${value}" to be kebab-case`,
			},
		],
		"custom-property-no-missing-var-function": true,
		"custom-property-pattern": [
			"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$",
			{
				message: (value) => `Expected custom property value "${value}" to be kebab-case`,
			},
		],
		"declaration-block-no-duplicate-custom-properties": true,
		"declaration-block-no-duplicate-properties": [
			true,
			{
				ignore: ["consecutive-duplicates-with-different-syntaxes"],
			},
		],
		"declaration-block-no-redundant-longhand-properties": true,
		"declaration-block-no-shorthand-property-overrides": true,
		"declaration-block-single-line-max-declarations": 1,
		"declaration-empty-line-before": [
			"always",
			{
				except: ["after-declaration", "first-nested"],
				ignore: ["after-comment", "inside-single-line-block"],
			},
		],
		"font-family-name-quotes": "always-where-recommended",
		"font-family-no-duplicate-names": true,
		"font-family-no-missing-generic-family-keyword": true,
		"function-calc-no-unspaced-operator": true,
		"function-disallowed-list": ["hsl", "hsla"],
		"function-linear-gradient-no-nonstandard-direction": true,
		"function-name-case": "lower",
		"function-no-unknown": null,
		"function-url-quotes": "always",
		"hue-degree-notation": "number",
		"import-notation": "url",
		"keyframe-block-no-duplicate-selectors": true,
		"keyframe-declaration-no-important": true,
		"keyframe-selector-notation": "percentage",
		"keyframes-name-pattern": [
			"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$",
			{
				message: (value) => `Expected keyframe value "${value}" to be kebab-case`,
			},
		],
		"length-zero-no-unit": [
			true,
			{
				ignore: ["custom-properties"],
			},
		],
		"max-nesting-depth": [
			2,
			{
				ignore: ["pseudo-classes"],
			},
		],
		"media-feature-name-no-unknown": true,
		"media-feature-name-no-vendor-prefix": true,
		"media-feature-range-notation": "context",
		"media-query-no-invalid": null,
		"named-grid-areas-no-invalid": true,
		"no-descending-specificity": null,
		"no-duplicate-at-import-rules": true,
		"no-duplicate-selectors": true,
		"no-invalid-double-slash-comments": true,
		"no-invalid-position-at-import-rule": null,
		"number-max-precision": 4,
		"order/order": [
			[
				{
					name: "include",
					type: "at-rule",
				},
			],
			{
				disableFix: true,
			},
		],
		"order/properties-alphabetical-order": null,
		"property-no-unknown": [
			true,
			{
				ignoreSelectors: [":export"],
			},
		],
		"property-no-vendor-prefix": true,
		"rule-empty-line-before": [
			"always-multi-line",
			{
				except: ["first-nested"],
				ignore: ["after-comment"],
			},
		],
		"scss/at-rule-no-unknown": true,
		"scss/operator-no-unspaced": true,
		"selector-anb-no-unmatchable": true,
		"selector-attribute-quotes": "always",
		"selector-class-pattern":
			// eslint-disable-next-line max-len
			"([A-Za-z0-9]+(?:-[A-Za-z0-9]+)*)(?:__([A-Za-z0-9]+(?:-[A-Za-z0-9]+)*))?(?:--([A-Za-z0-9]+(?:-[A-Za-z0-9]+)*))?",
		"selector-id-pattern": [
			"^([a-z][a-z0-9]*)(-[a-z0-9]+)*$",
			{
				message: (selector) => `Expected id selector "${selector}" to be kebab-case`,
			},
		],
		"selector-max-id": 1,
		"selector-no-qualifying-type": null,
		"selector-no-vendor-prefix": true,
		"selector-not-notation": "complex",
		"selector-pseudo-class-no-unknown": [
			true,
			{
				ignorePseudoClasses: ["global"],
			},
		],
		"selector-pseudo-element-colon-notation": "double",
		"selector-pseudo-element-no-unknown": true,
		"selector-type-case": "lower",
		"selector-type-no-unknown": [
			true,
			{
				ignore: ["custom-elements"],
			},
		],
		"shorthand-property-no-redundant-values": true,
		"string-no-newline": true,
		"unit-no-unknown": true,
		"value-keyword-case": "lower",
		"value-no-vendor-prefix": true,
	},
};
