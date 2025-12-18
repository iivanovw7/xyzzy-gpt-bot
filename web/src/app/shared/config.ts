import { mergeDeepRight } from "ramda";

import type { LoggerConfig } from "./logger";

import { environment } from "../../../environments/environment";
import { LogLevel } from "./logger";

export type TConfigEnv = {
	reconfig?: {
		logger?: {
			logColors?: boolean;
			logLevel?: LogLevel;
		};
	};
};

const DEFAULT_CONFIG = {
	logger: {
		logColors: true,
		logLevel: LogLevel.DEBUG,
		logPrefix: "[web]",
	},
	net: {
		requestTimeout: 20 * 1000,
		tokenRefreshPeriod: 60 * 1000,
	},
	ui: {
		debounce: 100,
		showTransactionDirection: true,
		throttle: 100,
	},
};

export const getConfig = (env: TConfigEnv) => {
	return mergeDeepRight(DEFAULT_CONFIG, env.reconfig || {}) as typeof DEFAULT_CONFIG;
};

export const config = getConfig({
	reconfig: {
		logger: {
			logColors: !environment.production,
			logLevel: environment.logLevel as LoggerConfig["level"],
		},
	},
});
