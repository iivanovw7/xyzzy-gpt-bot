import { storage } from "./storage";
import { StorageKey } from "./storage.types";

export type TokenStorage = typeof tokenStorage;

export const tokenStorage = {
	getAccessToken: (): null | string => {
		return storage.get(StorageKey.AccessToken);
	},

	removeAccessToken: () => {
		storage.delete(StorageKey.AccessToken);
	},

	setAccessToken: (token: null | string) => {
		storage.set(StorageKey.AccessToken, token);
	},
};
