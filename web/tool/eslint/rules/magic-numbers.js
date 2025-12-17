export const magicNumbersRules = {
	"no-magic-numbers": [
		"error",
		{
			enforceConst: true,
			ignore: [-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 24, 60, 100, 1000],
			ignoreArrayIndexes: true,
			ignoreDefaultValues: true,
		},
	],
};
