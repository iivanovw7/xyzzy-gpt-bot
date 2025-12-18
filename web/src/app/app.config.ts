import type { ApplicationConfig } from "@angular/core";

import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { inject, provideAppInitializer, provideBrowserGlobalErrorListeners } from "@angular/core";
import { provideRouter } from "@angular/router";
import { firstValueFrom } from "rxjs";

import { routes } from "./app.routes";
import { AuthService } from "./core/auth/services/auth.service";
import { apiInterceptor } from "./core/interceptors/api.interceptor";
import { errorInterceptor } from "./core/interceptors/error.interceptor";
import { tokenInterceptor } from "./core/interceptors/token.interceptor";
import { config } from "./shared/config";
import { logger } from "./shared/logger";
import { ThemeService } from "./shared/services/theme.service";

export const initAuth = (authService: AuthService) => {
	return async () => {
		let authResult$ = authService.getAccessToken() ? authService.refreshToken() : authService.login();

		try {
			await firstValueFrom(authResult$, { defaultValue: null });
		} catch (error) {
			logger.error("Auth initialization failed", error);
		}
	};
};

export const appConfig: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
		provideAppInitializer(() => {
			logger.configure({
				enableColors: config.logger.logColors,
				level: config.logger.logLevel,
				prefix: config.logger.logPrefix,
			});

			let authInitializer = initAuth(inject(AuthService));
			let themeService = inject(ThemeService);

			themeService.initialize();

			return authInitializer();
		}),
	],
};
