import log from "loglevel";

import type { LoggerConfig } from "./logger.types";

import { LogLevel } from "./logger.types";

class Logger {
	private config: LoggerConfig = {
		level: LogLevel.DEBUG,
		prefix: "",
	};

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

	public configure(config: LoggerConfig) {
		this.config = { ...config };
		log.setLevel(config.level);
		this.patchColors();
		this.info("Loglevel: ", this.config.level);
	}

	debug(message: unknown, ...arguments_: unknown[]) {
		log.debug(`${this.config.prefix}: ${message}`, ...arguments_);
	}
	error(message: unknown, ...arguments_: unknown[]) {
		log.error(`${this.config.prefix}: ${message}`, ...arguments_);
	}
	info(message: unknown, ...arguments_: unknown[]) {
		log.info(`${this.config.prefix}: ${message}`, ...arguments_);
	}
	warn(message: unknown, ...arguments_: unknown[]) {
		log.warn(`${this.config.prefix}: ${message}`, ...arguments_);
	}
}

export const logger = new Logger();
