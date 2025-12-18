import { signal } from "@angular/core";

import type { StorageKey, StorageValue } from "./storage.types";

import { logger } from "../logger";
import { DEFAULT_STORAGE } from "./storage.types";

class Storage {
	private readonly STORAGE_KEY = "app-storage";
	private readonly store = signal<StorageValue>(this.loadInitial());
	public readonly state = this.store.asReadonly();

	private loadInitial(): StorageValue {
		try {
			let raw = localStorage.getItem(this.STORAGE_KEY);

			if (!raw) return { ...(DEFAULT_STORAGE as StorageValue) };

			return JSON.parse(raw) as StorageValue;
		} catch (error) {
			logger.error("[Storage] Deserialize error", error);

			return { ...(DEFAULT_STORAGE as StorageValue) };
		}
	}

	private save(value: StorageValue): void {
		try {
			localStorage.setItem(this.STORAGE_KEY, JSON.stringify(value));
		} catch (error) {
			logger.error("[Storage] Serialize error", error);
		}
	}

	clear(): void {
		let newState = { ...(DEFAULT_STORAGE as StorageValue) };
		this.store.set(newState);
		this.save(newState);
	}

	delete<K extends StorageKey>(key: K): void {
		this.store.update((currentState) => {
			let newState = { ...currentState };
			delete newState[key];
			this.save(newState);

			return newState;
		});
	}

	get<K extends StorageKey>(key: K): StorageValue[K] {
		let value = this.store()[key];

		return value ?? (DEFAULT_STORAGE[key] || null);
	}

	patch(value: Partial<StorageValue>): void {
		this.store.update((s) => {
			let newState = { ...s, ...value };
			this.save(newState);

			return newState;
		});
	}

	set<K extends StorageKey>(key: K, value: StorageValue[K]): void {
		this.store.update((s) => {
			let newState = { ...s, [key]: value };
			this.save(newState);

			return newState;
		});
	}
}

export const storage = new Storage();
