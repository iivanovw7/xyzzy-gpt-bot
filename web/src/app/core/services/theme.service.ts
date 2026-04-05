import type { Theme } from "@/types/styles.types";

import { themeStorage } from "@/app/shared/storage/theme.storage";
import { Injectable, signal } from "@angular/core";

@Injectable({
	providedIn: "root",
})
export class ThemeService {
	private readonly themeSignal = signal<Theme>(this.getInitialTheme());
	public readonly theme = this.themeSignal.asReadonly();

	constructor() {
		this.applyTheme(this.themeSignal());
	}

	private applyTheme(theme: Theme): void {
		document.documentElement.dataset["theme"] = theme;
	}

	private getInitialTheme(): Theme {
		let savedTheme = themeStorage.getTheme();

		if (savedTheme === "dark" || savedTheme === "light") {
			return savedTheme as Theme;
		}

		return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
	}

	public setTheme(theme: Theme): void {
		this.themeSignal.set(theme);
		this.applyTheme(theme);
		themeStorage.setTheme(theme);
	}
}
