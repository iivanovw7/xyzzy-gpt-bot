import type { HttpErrorResponse, HttpEvent, HttpHandlerFn, HttpInterceptorFn, HttpRequest } from "@angular/common/http";
import type { Observable } from "rxjs";

import { HttpStatusCode } from "@angular/common/http";
import { inject } from "@angular/core";
import { throwError } from "rxjs";
import { catchError, filter, switchMap, take } from "rxjs/operators";

import { AuthService } from "../auth/service";

const withToken = (request: HttpRequest<unknown>, token?: null | string) => {
	if (token) {
		return request.clone({
			setHeaders: { Authorization: `Bearer ${token}` },
		});
	}

	return request;
};

const authError = (auth: AuthService, error?: unknown) => {
	auth.logout();

	return throwError(() => error ?? new Error("Token refresh failed"));
};

export const tokenInterceptor: HttpInterceptorFn = (
	request: HttpRequest<unknown>,
	next: HttpHandlerFn,
): Observable<HttpEvent<unknown>> => {
	let authService = inject(AuthService);

	let token = authService.getAccessToken();

	let authRequest = withToken(request, token);

	return next(authRequest).pipe(
		catchError((error: HttpErrorResponse) => {
			if (error.status !== HttpStatusCode.Unauthorized || authRequest.url.includes("/refresh") || !token) {
				return throwError(() => error);
			}

			if (!authService.isRefreshing) {
				return authService.refreshToken().pipe(
					switchMap((response) => {
						let newToken = response?.accessToken;

						return newToken ? next(withToken(authRequest, newToken)) : authError(authService);
					}),
					catchError((error_) => authError(authService, error_)),
				);
			}

			return authService.refreshToken$.pipe(
				filter(Boolean),
				take(1),
				switchMap((newToken) => next(withToken(authRequest, newToken))),
			);
		}),
	);
};
