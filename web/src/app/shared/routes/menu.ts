import { pipe, prop, sortBy, values } from "ramda";

import type { IconKey } from "../ui/components/icon";

import { routePath } from "./routes";

const { budgeting, home, settings } = routePath;

export type MenuItem = {
	disabled?: boolean;
	icon?: IconKey;
	order: number;
	replace?: boolean;
	text: string;
	to: string;
};

export const menuItemSet: Record<string, MenuItem> = {
	budgeting: {
		icon: "Wallet",
		order: 1,
		text: "Budgeting",
		to: budgeting,
	},
	home: {
		icon: "House",
		order: 0,
		text: "Accounts",
		to: home,
	},
	settings: {
		disabled: true,
		icon: "Settings",
		order: 2,
		text: "Settings",
		to: settings,
	},
};

const sortByOrder = sortBy(prop("order"));

export const menuItems: MenuItem[] = pipe(values, sortByOrder)(menuItemSet);
