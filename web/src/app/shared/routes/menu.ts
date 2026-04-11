import { pipe, prop, sortBy, values } from "ramda";

import type { IconKey } from "../ui/components/icon";

import { routePath } from "./routes";

const { budgeting, home, links, settings } = routePath;

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
	links: {
		icon: "Link",
		order: 2,
		text: "Links",
		to: links,
	},
	settings: {
		icon: "Settings",
		order: 3,
		text: "Settings",
		to: settings,
	},
};

const sortByOrder = sortBy(prop("order"));

export const menuItems: MenuItem[] = pipe(values, sortByOrder)(menuItemSet);
