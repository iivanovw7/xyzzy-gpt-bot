import type { LogLevelDesc } from "loglevel";

import log from "loglevel";

import type { LoggerConfig } from "./logger.types";

import { DEFAULT_LOGGER_CONFIG } from "./logger.config";

class Logger {
	private config: LoggerConfig = DEFAULT_LOGGER_CONFIG;

	private getColor(method: string): string {
		let colors: Record<string, string> = {
			debug: "#03A9F4",
			error: "#F44336",
			info: "#4CAF50",
			trace: "#9E9E9E",
			warn: "#FFC107",
		};

		return colors[method] || "inherit";
	}

	private patchColors() {
		if (!this.config.enableColors) return;

		let originalFactory = log.methodFactory;

		log.methodFactory = (methodName, logLevel, loggerName) => {
			let handler = originalFactory(methodName, logLevel, loggerName);
			let color = this.getColor(methodName);

			return (...arguments_: unknown[]) => {
				if (typeof arguments_[0] === "string") {
					handler(`%c${arguments_[0]}`, `color: ${color}; font-weight: bold`, ...arguments_.slice(1));
				} else {
					handler(...arguments_);
				}
			};
		};
		log.setLevel(log.getLevel());
	}

	public configure(customConfig: Partial<LoggerConfig>) {
		this.config = { ...DEFAULT_LOGGER_CONFIG, ...customConfig };
		log.setLevel(this.config.level.toLocaleLowerCase() as LogLevelDesc);
		this.patchColors();
	}

	debug(message: unknown, ...arguments_: unknown[]) {
		log.debug(message, ...arguments_);
	}
	error(message: unknown, ...arguments_: unknown[]) {
		log.error(message, ...arguments_);
	}
	info(message: unknown, ...arguments_: unknown[]) {
		log.info(message, ...arguments_);
	}
	warn(message: unknown, ...arguments_: unknown[]) {
		log.warn(message, ...arguments_);
	}
}

export const logger = new Logger();
