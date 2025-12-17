import { inject, Injectable } from "@angular/core";

import type { Theme } from "../../../types/styles.types";

import { StorageService } from "./storage.service";
import { StorageKey } from "./storage.types";

@Injectable({ providedIn: "root" })
export class ThemeStorage {
	private storage = inject(StorageService);

	getTheme(): Nullable<Theme> {
		return this.storage.get(StorageKey.Theme);
	}

	setTheme(theme: Nullable<Theme>) {
		this.storage.set(StorageKey.Theme, theme);
	}
}
