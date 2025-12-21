import { map } from "ramda";

export const basePath = {
	budgeting: "budgeting",
	home: "",
	login: "login",
	settings: "settings",
} as const;

const toRoutePath = (path: string) => (path ? `/${path}` : "/");

const createRoutePath = <T extends Record<string, string>>(paths: T) => {
	return map(toRoutePath, paths) as { [K in keyof T]: string };
};

export const routePath = createRoutePath(basePath);
