import { getFromTelegram, removeFromTelegram, setToTelegram, storage } from "./storage";
import { StorageKey } from "./storage.types";

export type TokenStorage = typeof tokenStorage;

export const tokenStorage = {
	getAccessToken: (): null | string => storage.get(StorageKey.AccessToken),

	initialize: async (): Promise<void> => {
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
