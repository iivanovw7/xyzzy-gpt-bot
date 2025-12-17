import { ApplicationConfig, inject, provideAppInitializer, provideBrowserGlobalErrorListeners } from "@angular/core";
import { provideRouter } from "@angular/router";

import { routes } from "./app.routes";
import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { apiInterceptor } from "./core/interceptors/api.interceptor";
import { tokenInterceptor } from "./core/interceptors/token.interceptor";
import { errorInterceptor } from "./core/interceptors/error.interceptor";
import { ThemeService } from "./shared/services/theme.service";
import { AuthService } from "./core/auth/services/auth.service";

export const initAuth = (authService: AuthService) => {
	return () => {
		if (authService.getAccessToken()) {
			console.log("Existing session found. App initialized.");

			return authService.refreshToken();
		}

		console.log("No existing session found.");

		return authService.login();
	};
};

export const appConfig: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
		provideAppInitializer(() => {
			let authInitializer = initAuth(inject(AuthService));
			let themeService = inject(ThemeService);

			themeService.initialize();

			return authInitializer();
		}),
	],
};
