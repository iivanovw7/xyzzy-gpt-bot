import type { ApplicationConfig } from "@angular/core";

import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { inject, provideAppInitializer, provideBrowserGlobalErrorListeners } from "@angular/core";
import { provideRouter } from "@angular/router";

import type { LoggerConfig } from "./shared/logger";

import { environment } from "../../environments/environment";
import { routes } from "./app.routes";
import { AuthService } from "./core/auth/services/auth.service";
import { apiInterceptor } from "./core/interceptors/api.interceptor";
import { errorInterceptor } from "./core/interceptors/error.interceptor";
import { tokenInterceptor } from "./core/interceptors/token.interceptor";
import { logger } from "./shared/logger";
import { ThemeService } from "./shared/services/theme.service";

export const initAuth = (authService: AuthService) => {
	return () => {
		if (authService.getAccessToken()) {
			logger.info("Existing session found. App initialized.");

			return authService.refreshToken();
		}

		logger.info("No existing session found.");

		return authService.login();
	};
};

export const appConfig: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
		provideAppInitializer(() => {
			logger.configure({
				enableColors: !environment.production,
				level: environment.logLevel as LoggerConfig["level"],
				prefix: "[web]",
			});

			let authInitializer = initAuth(inject(AuthService));
			let themeService = inject(ThemeService);

			themeService.initialize();

			return authInitializer();
		}),
	],
};
