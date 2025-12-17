import type { LoggerConfig } from "./logger.types";

export const DEFAULT_LOGGER_CONFIG: LoggerConfig = {
	enableColors: true,
	level: "INFO",
	prefix: "[web]",
};
