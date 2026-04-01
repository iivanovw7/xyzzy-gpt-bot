import { all } from "ramda";

export const allKeyValuesZero = <T, K extends keyof T>(items: T[], key: K): boolean => {
	return all<T>((item) => item[key] === 0, items);
};
