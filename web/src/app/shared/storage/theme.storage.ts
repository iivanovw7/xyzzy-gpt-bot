import type { Theme } from "../../../types/styles.types";

import { storage } from "./storage";
import { StorageKey } from "./storage.types";

export type ThemeStorage = typeof themeStorage;

export const themeStorage = {
	getTheme: (): Nullable<Theme> => {
		return storage.get(StorageKey.Theme);
	},

	setTheme: (theme: Nullable<Theme>) => {
		storage.set(StorageKey.Theme, theme);
	},
};
