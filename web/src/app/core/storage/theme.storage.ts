import { Injectable, inject } from "@angular/core";
import { StorageService } from "./storage.service";
import { StorageKey } from "./storage.types";
import { Theme } from "../../../types/styles.types";

@Injectable({ providedIn: "root" })
export class ThemeStorage {
	private storage = inject(StorageService);

	setTheme(theme: Nullable<Theme>) {
		this.storage.set(StorageKey.Theme, theme);
	}

	getTheme(): Nullable<Theme> {
		return this.storage.get(StorageKey.Theme);
	}
}
