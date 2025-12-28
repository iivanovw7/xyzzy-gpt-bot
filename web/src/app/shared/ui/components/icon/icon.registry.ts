export const Icon = {
	House: "@tui.house",
	LogOut: "@tui.log-out",
	Settings: "@tui.settings",
	User: "@tui.user",
	Wallet: "@tui.wallet",
} as const;

export type IconKey = keyof typeof Icon;
