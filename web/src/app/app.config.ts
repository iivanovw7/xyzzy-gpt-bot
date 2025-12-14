import {
    ApplicationConfig,
    inject,
    provideAppInitializer,
    provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideRouter, Router } from '@angular/router';

import { routes } from './app.routes';
import { JwtService } from './core/auth/services/jwt.service';
import { UserService } from './core/auth/services/user.service';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { apiInterceptor } from './core/interceptors/api.interceptor';
import { tokenInterceptor } from './core/interceptors/token.interceptor';
import { errorInterceptor } from './core/interceptors/error.interceptor';

export function initAuth(jwtService: JwtService, userService: UserService, router: Router) {
    return () => {
        return jwtService.getToken() ? userService.getCurrentUser() : router.navigate(['login']);
    };
}

export const appConfig: ApplicationConfig = {
    providers: [
        provideBrowserGlobalErrorListeners(),
        provideRouter(routes),
        provideHttpClient(withInterceptors([apiInterceptor, tokenInterceptor, errorInterceptor])),
        provideAppInitializer(() => {
            let intializeerFn = initAuth(inject(JwtService), inject(UserService), inject(Router));

            return intializeerFn();
        }),
    ],
};
