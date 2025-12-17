import { Theme } from "../../../types/styles.types";

export const StorageKey = {
	Theme: "theme",
	AccessToken: "accessToken",
} as const;

export type StorageKey = (typeof StorageKey)[keyof typeof StorageKey];

export type StorageValue = {
	[StorageKey.Theme]: Nullable<Theme>;
	[StorageKey.AccessToken]: Nullable<string>;
};

export const DEFAULT_STORAGE: Partial<StorageValue> = {
	[StorageKey.Theme]: null,
};
