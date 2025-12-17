import { Injectable } from "@angular/core";

import type { Theme } from "../../../types/styles.types";

import { env } from "../env";
import { themeStorage } from "../storage";

@Injectable({ providedIn: "root" })
export class ThemeService {
	initialize() {
		this.setTheme(themeStorage.getTheme() || env.isDarkTheme ? "dark" : "light");
	}

	setTheme(theme: Theme) {
		document.documentElement.dataset["theme"] = theme;
		themeStorage.setTheme(theme);
	}
}
