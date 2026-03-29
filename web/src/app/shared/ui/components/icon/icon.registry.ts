export const Icon = {
	Check: "@tui.check",
	House: "@tui.house",
	LogOut: "@tui.log-out",
	MoveUp: "@tui.move-up",
	Settings: "@tui.settings",
	User: "@tui.user",
	Wallet: "@tui.wallet",
} as const;

export type IconKey = keyof typeof Icon;
