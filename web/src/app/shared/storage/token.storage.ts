import { storage } from "./storage";
import { StorageKey } from "./storage.types";

export type TokenStorage = typeof tokenStorage;

export const tokenStorage = {
	getAccessToken: (): Nullable<string> => {
		return storage.get(StorageKey.AccessToken);
	},

	removeAccessToken: () => {
		storage.delete(StorageKey.AccessToken);
	},

	setAccessToken: (token: Nullable<string>) => {
		storage.set(StorageKey.AccessToken, token);
	},
};
