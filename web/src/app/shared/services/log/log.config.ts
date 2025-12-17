import { InjectionToken } from "@angular/core";

import type { LoggerConfig } from "./log.types";

export const LOGGER_CONFIG = new InjectionToken<LoggerConfig>("LOGGER_CONFIG");

export const DEFAULT_LOGGER_CONFIG: LoggerConfig = {
	enableColors: true,
	level: "INFO",
	prefix: "[web]",
};
