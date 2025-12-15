import { Injectable, inject } from "@angular/core";
import { StorageService } from "./storage.service";
import { StorageKey } from "./storage.types";
import { Theme } from "../../shared/types/styles.types";

@Injectable({ providedIn: "root" })
export class ThemeStorage {
	private storage = inject(StorageService);

	setTheme(theme: Theme | null) {
		this.storage.set(StorageKey.Theme, theme);
	}

	getTheme(): Theme | null {
		return this.storage.get(StorageKey.Theme);
	}
}
