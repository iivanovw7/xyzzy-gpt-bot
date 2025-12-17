import { inject, Injectable } from "@angular/core";
import { ThemeStorage } from "../../core/storage/theme.storage";
import { Theme } from "../../../types/styles.types";
import { env } from "../env";

@Injectable({ providedIn: "root" })
export class ThemeService {
	themeStorage = inject(ThemeStorage);

	initialize() {
		this.setTheme(this.themeStorage.getTheme() || env.isDarkTheme ? "dark" : "light");
	}

	setTheme(theme: Theme) {
		document.documentElement.dataset["theme"] = theme;
		this.themeStorage.setTheme(theme);
	}
}
