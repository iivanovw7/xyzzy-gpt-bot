import type { ApplicationConfig } from "@angular/core";
import type { LoginResponse } from "@bindings";
import type { Observable } from "rxjs";

import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { inject, provideAppInitializer, provideBrowserGlobalErrorListeners } from "@angular/core";
import { provideRouter } from "@angular/router";
import { provideEventPlugins } from "@taiga-ui/event-plugins";
import { firstValueFrom } from "rxjs";

import { routes } from "./app.routes";
import { AuthService } from "./core/auth";
import { apiInterceptor, errorInterceptor, tokenInterceptor } from "./core/interceptors";
import { config } from "./shared/config";
import { env } from "./shared/env";
import { logger } from "./shared/logger";
import { tokenStorage } from "./shared/storage";

export const initAuth = (authService: AuthService) => {
	return async () => {
		let accessToken = authService.getAccessToken();
		let authResult$: Observable<Nullable<LoginResponse>>;

		if (accessToken) {
			authResult$ = authService.refreshToken();
		} else if (env.telegramInitData) {
			authResult$ = authService.login();
		} else {
			return;
		}

		try {
			await firstValueFrom(authResult$, { defaultValue: null });
		} catch (error) {
			logger.error("Auth initialization failed", error);
		}
	};
};

export const appConfig: ApplicationConfig = {
	providers: [
		provideEventPlugins(),
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
		provideAppInitializer(async () => {
			logger.configure({
				enableColors: config.logger.logColors,
				level: config.logger.logLevel,
				prefix: config.logger.logPrefix,
			});

			let authInitializer = initAuth(inject(AuthService));

			await tokenStorage.initialize();

			return authInitializer();
		}),
		provideEventPlugins(),
	],
};
