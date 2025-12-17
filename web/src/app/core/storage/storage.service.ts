import { effect, inject, Injectable, signal } from "@angular/core";

import type { StorageKey, StorageValue } from "./storage.types";

import { LoggerService } from "../../shared/services/log/log.service";
import { DEFAULT_STORAGE } from "./storage.types";

@Injectable({ providedIn: "root" })
export class StorageService {
	private log = inject(LoggerService);

	private readonly store = signal<StorageValue>(this.load() ?? { ...(DEFAULT_STORAGE as StorageValue) });

	constructor() {
		effect(() => {
			localStorage.setItem("app-storage", JSON.stringify(this.store()));
		});
	}

	private load(): null | StorageValue {
		try {
			let raw = localStorage.getItem("app-storage");

			return raw ? JSON.parse(raw) : null;
		} catch (error) {
			this.log.error("[Storage] Deserialize error", error);

			return null;
		}
	}

	delete<K extends StorageKey>(key: K): void {
		this.store.update((currentState) => {
			let { [key]: _, ...rest } = currentState;

			return rest as StorageValue;
		});
	}

	get<K extends StorageKey>(key: K): StorageValue[K] {
		return this.store()[key];
	}

	patch(value: Partial<StorageValue>) {
		this.store.update((s) => ({ ...s, ...value }));
	}

	set<K extends StorageKey>(key: K, value: StorageValue[K]) {
		this.store.update((s) => ({
			...s,
			[key]: value,
		}));
	}
}
