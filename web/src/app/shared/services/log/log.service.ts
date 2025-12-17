import type { LogLevelDesc } from "loglevel";

import { inject, Injectable } from "@angular/core";
import log from "loglevel";

import type { LoggerConfig } from "./log.types";

import { DEFAULT_LOGGER_CONFIG, LOGGER_CONFIG } from "./log.config";

@Injectable({
	providedIn: "root",
})
export class LoggerService {
	private readonly injectedConfig = inject(LOGGER_CONFIG, { optional: true });

	private readonly config: LoggerConfig = {
		...DEFAULT_LOGGER_CONFIG,
		...(this.injectedConfig ?? {}),
	};

	constructor() {
		this.initializeLogger();
	}

	private format(message: unknown): unknown {
		if (typeof message === "string" && this.config.prefix) {
			return `${this.config.prefix} ${message}`;
		}

		return message;
	}

	private getColor(method: string): string {
		switch (method) {
			case "debug":
				return "#03A9F4";
			case "error":
				return "#F44336";
			case "info":
				return "#4CAF50";
			case "trace":
				return "#9E9E9E";
			case "warn":
				return "#FFC107";
			default:
				return "inherit";
		}
	}

	private initializeLogger() {
		log.setLevel(this.config.level.toLocaleLowerCase() as LogLevelDesc);
		this.patchColors();

		let mode = this.config.level === "ERROR" ? "PRODUCTION" : "DEVELOPMENT";

		log.info(`%cLogger initialized`, "font-weight: bold", {
			colors: this.config.enableColors,
			level: this.config.level,
			mode,
		});
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

	debug(message: unknown, ...arguments_: unknown[]) {
		log.debug(this.format(message), ...arguments_);
	}

	error(message: unknown, ...arguments_: unknown[]) {
		log.error(this.format(message), ...arguments_);
	}

	info(message: unknown, ...arguments_: unknown[]) {
		log.info(this.format(message), ...arguments_);
	}

	trace(message: unknown, ...arguments_: unknown[]) {
		log.trace(this.format(message), ...arguments_);
	}

	warn(message: unknown, ...arguments_: unknown[]) {
		log.warn(this.format(message), ...arguments_);
	}
}
