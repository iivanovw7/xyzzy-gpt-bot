export const Icon = {
	BanknoteArrowDown: "@tui.banknote-arrow-down",
	BanknoteArrowUp: "@tui.banknote-arrow-up",
	Check: "@tui.check",
	Cloud: "@tui.cloud",
	CreditCard: "@tui.credit-card",
	House: "@tui.house",
	Info: "@tui.info",
	Link: "@tui.link",
	LogOut: "@tui.log-out",
	MoveUp: "@tui.move-up",
	Settings: "@tui.settings",
	User: "@tui.user",
	Wallet: "@tui.wallet",
} as const;

export type IconKey = keyof typeof Icon;
