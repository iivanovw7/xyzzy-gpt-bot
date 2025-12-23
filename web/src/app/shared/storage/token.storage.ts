import { storage } from "./storage";
import { StorageKey } from "./storage.types";

export type TokenStorage = typeof tokenStorage;

export const tokenStorage = {
	getAccessToken: (): Nullable<string> => {
		let tgToken = window.Telegram?.WebApp?.CloudStorage.getItem(StorageKey.AccessToken);

		alert("get access token from tg: " + String(tgToken));

		return tgToken ?? storage.get(StorageKey.AccessToken);
	},

	removeAccessToken: () => {
		storage.delete(StorageKey.AccessToken);
		window.Telegram?.WebApp?.CloudStorage.removeItem(StorageKey.AccessToken);
	},

	setAccessToken: (token: Nullable<string>) => {
		storage.set(StorageKey.AccessToken, token);
		window.Telegram?.WebApp?.CloudStorage.setItem(StorageKey.AccessToken, token);
	},

	syncFromCloud: (): Promise<void> => {
		return new Promise((resolve) => {
			let tg = window.Telegram?.WebApp;

			if (!tg) {
				return resolve();
			}

			tg.ready();

			return tg.CloudStorage.getItem(StorageKey.AccessToken, (error: unknown, value: unknown) => {
				if (!error && typeof value === "string") {
					storage.set(StorageKey.AccessToken, value);
				}
				resolve();
			});
		});
	},
};
