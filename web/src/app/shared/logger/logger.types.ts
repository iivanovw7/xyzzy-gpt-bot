export type LogLevel = "DEBUG" | "ERROR" | "INFO" | "SILENT" | "TRACE" | "WARN";

export type LoggerConfig = {
	enableColors?: boolean;
	level: LogLevel;
	prefix?: string;
};
