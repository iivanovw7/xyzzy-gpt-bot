import type { LogLevelDesc, LogLevel as LogLevelOptions } from "loglevel";

export const LogLevel = {
	DEBUG: "debug",
	ERROR: "error",
	INFO: "info",
	SILENT: "silent",
	TRACE: "trace",
	WARN: "warn",
} as const satisfies { [Key in keyof LogLevelOptions]: LogLevelDesc };

export type LogLevel = (typeof LogLevel)[keyof typeof LogLevel];

export type LoggerConfig = {
	enableColors?: boolean;
	level: LogLevel;
	prefix: string;
};
