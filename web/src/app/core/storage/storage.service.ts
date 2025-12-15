import { Injectable, signal, effect } from "@angular/core";
import { StorageKey, StorageValue, DEFAULT_STORAGE } from "./storage.types";

@Injectable({ providedIn: "root" })
export class StorageService {
	private readonly store = signal<StorageValue>(this.load() ?? { ...DEFAULT_STORAGE });

	constructor() {
		effect(() => {
			localStorage.setItem("app-storage", JSON.stringify(this.store()));
		});
	}

	get<K extends StorageKey>(key: K): StorageValue[K] {
		return this.store()[key];
	}

	set<K extends StorageKey>(key: K, value: StorageValue[K]) {
		this.store.update((s) => ({
			...s,
			[key]: value,
		}));
	}

	patch(value: Partial<StorageValue>) {
		this.store.update((s) => ({ ...s, ...value }));
	}

	private load(): StorageValue | null {
		try {
			const raw = localStorage.getItem("app-storage");
			return raw ? JSON.parse(raw) : null;
		} catch (e) {
			console.error("[Storage] Deserialize error", e);
			return null;
		}
	}
}
