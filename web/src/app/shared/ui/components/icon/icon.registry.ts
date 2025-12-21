import { House, LogOut, Settings, User, Wallet } from "lucide-angular";

export const Icon = {
	House,
	LogOut,
	Settings,
	User,
	Wallet,
} as const;

export type IconKey = keyof typeof Icon;
