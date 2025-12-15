import { Theme } from "../../shared/types/styles.types";

export const StorageKey = {
	Theme: "theme",
} as const;

export type StorageKey = (typeof StorageKey)[keyof typeof StorageKey];

export type StorageValue = {
	[StorageKey.Theme]: Theme | null;
};

export const DEFAULT_STORAGE: StorageValue = {
	[StorageKey.Theme]: null,
};
