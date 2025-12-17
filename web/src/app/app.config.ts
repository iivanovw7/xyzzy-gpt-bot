import type { ApplicationConfig } from "@angular/core";

import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { inject, provideAppInitializer, provideBrowserGlobalErrorListeners } from "@angular/core";
import { provideRouter } from "@angular/router";

import { environment } from "../../environments/environment";
import { routes } from "./app.routes";
import { AuthService } from "./core/auth/services/auth.service";
import { apiInterceptor } from "./core/interceptors/api.interceptor";
import { errorInterceptor } from "./core/interceptors/error.interceptor";
import { tokenInterceptor } from "./core/interceptors/token.interceptor";
import { LOGGER_CONFIG } from "./shared/services/log/log.config";
import { LoggerService } from "./shared/services/log/log.service";
import { ThemeService } from "./shared/services/theme.service";

export const initAuth = (authService: AuthService, log: LoggerService) => {
	return () => {
		if (authService.getAccessToken()) {
			log.info("Existing session found. App initialized.");

			return authService.refreshToken();
		}

		log.info("No existing session found.");

		return authService.login();
	};
};

export const appConfig: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
		{
			provide: LOGGER_CONFIG,
			useValue: {
				enableColors: !environment.production,
				level: environment.logLevel,
				prefix: "[web]",
			},
		},
		provideAppInitializer(() => {
			let authInitializer = initAuth(inject(AuthService), inject(LoggerService));
			let themeService = inject(ThemeService);

			themeService.initialize();

			return authInitializer();
		}),
	],
};
