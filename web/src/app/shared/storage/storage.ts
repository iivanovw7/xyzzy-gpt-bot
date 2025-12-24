import { signal } from "@angular/core";

import type { StorageKey, StorageValue } from "./storage.types";

import { logger } from "../logger";
import { DEFAULT_STORAGE } from "./storage.types";

export const isCloudStorageSupported = () => {
	let tg = window.Telegram?.WebApp;
	let version = typeof tg?.version === "string" ? parseFloat(tg.version) : 0;

	return Boolean(version >= 6.1 && !!tg?.CloudStorage);
};

type CloudStorageValue = null | string;

export const getFromTelegram = (key: string): Promise<CloudStorageValue> => {
	return new Promise((resolve) => {
		let tg = window.Telegram?.WebApp;

		if (!isCloudStorageSupported() || !tg) {
			return resolve(null);
		}

		return tg.CloudStorage?.getItem(key, (error: unknown, value?: string) => {
			if (error || typeof value !== "string") {
				resolve(null);
			} else {
				resolve(value);
			}
		});
	});
};

export const setToTelegram = (key: string, value: CloudStorageValue): Promise<void> => {
	return new Promise((resolve) => {
		let tg = window.Telegram?.WebApp;

		if (!isCloudStorageSupported() || !tg || value === null) {
			return resolve();
		}

		return tg.CloudStorage?.setItem(key, value, () => resolve());
	});
};

export const removeFromTelegram = (key: string): Promise<void> => {
	return new Promise((resolve) => {
		let tg = window.Telegram?.WebApp;

		if (!isCloudStorageSupported() || !tg) {
			return resolve();
		}

		return tg.CloudStorage?.removeItem(key, () => resolve());
	});
};

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
