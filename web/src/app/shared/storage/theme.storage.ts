import type { Theme } from "@/types/styles.types";

import { storage } from "./storage";
import { StorageKey } from "./storage.types";

export type ThemeStorage = typeof themeStorage;

export const themeStorage = {
	getTheme: (): null | string => storage.get(StorageKey.Theme),
	setTheme: (theme?: Maybe<Theme>): void => {
		storage.set(StorageKey.Theme, theme || null);
	},
};
