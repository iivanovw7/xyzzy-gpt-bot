/* eslint-disable consistent-return */
import { storage } from "./storage";
import { StorageKey } from "./storage.types";

export const isTelegramWebApp = (): boolean => Boolean(window.Telegram?.WebApp?.initData);

export const isCloudStorageSupported = (): boolean => {
	let tg = window.Telegram?.WebApp;

	return Boolean(tg && typeof tg.version === "string" && parseFloat(tg.version) >= 6.1 && tg.CloudStorage);
};

type CloudStorageValue = null | string;

export const getFromTelegram = (key: string): Promise<CloudStorageValue> =>
	new Promise((resolve) => {
		let tg = window.Telegram?.WebApp;

		if (!isCloudStorageSupported() || !tg) {
			return resolve(null);
		}

		tg.CloudStorage?.getItem(key, (error: unknown, value?: string) => {
			if (error || typeof value !== "string") {
				resolve(null);
			} else {
				resolve(value);
			}
		});
	});

export const setToTelegram = (key: string, value: CloudStorageValue): Promise<void> =>
	new Promise((resolve) => {
		let tg = window.Telegram?.WebApp;

		if (!isCloudStorageSupported() || !tg || value === null) {
			return resolve();
		}

		tg.CloudStorage?.setItem(key, value, () => resolve());
	});

export const removeFromTelegram = (key: string): Promise<void> =>
	new Promise((resolve) => {
		let tg = window.Telegram?.WebApp;

		if (!isCloudStorageSupported() || !tg) {
			return resolve();
		}

		tg.CloudStorage?.removeItem(key, () => resolve());
	});

export type TokenStorage = typeof tokenStorage;

export const tokenStorage = {
	getAccessToken: (): null | string => storage.get(StorageKey.AccessToken),

	init: async (): Promise<void> => {
		let token = await getFromTelegram(StorageKey.AccessToken);
		if (token) {
			storage.set(StorageKey.AccessToken, token);
		}
	},

	removeAccessToken: async (): Promise<void> => {
		storage.delete(StorageKey.AccessToken);
		await removeFromTelegram(StorageKey.AccessToken);
	},

	setAccessToken: async (token: null | string): Promise<void> => {
		storage.set(StorageKey.AccessToken, token);

		if (token) {
			await setToTelegram(StorageKey.AccessToken, token);
		}
	},
};
