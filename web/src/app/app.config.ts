import { ApplicationConfig, inject, provideAppInitializer, provideBrowserGlobalErrorListeners } from "@angular/core";
import { provideRouter } from "@angular/router";

import { routes } from "./app.routes";
import { JwtService } from "./core/auth/services/jwt.service";
import { UserService } from "./core/auth/services/user.service";
import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { apiInterceptor } from "./core/interceptors/api.interceptor";
import { tokenInterceptor } from "./core/interceptors/token.interceptor";
import { errorInterceptor } from "./core/interceptors/error.interceptor";
import { EMPTY } from "rxjs";
import { ThemeService } from "./shared/services/theme.service";

export const initAuth = (jwtService: JwtService, userService: UserService) => {
	return () => {
		return jwtService.getToken() ? userService.getCurrentUser() : EMPTY;
	};
};

export const appConfig: ApplicationConfig = {
	providers: [
		provideBrowserGlobalErrorListeners(),
		provideRouter(routes),
		provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
		provideAppInitializer(() => {
			let authInitializer = initAuth(inject(JwtService), inject(UserService));
			let themeService = inject(ThemeService);

			themeService.initialize();

			return authInitializer();
		}),
	],
};
