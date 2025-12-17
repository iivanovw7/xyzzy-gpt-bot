import { inject } from "@angular/core";
import { HttpRequest, HttpEvent, HttpErrorResponse, HttpInterceptorFn, HttpHandlerFn } from "@angular/common/http";
import { Observable, throwError } from "rxjs";
import { catchError, switchMap, filter, take } from "rxjs/operators";
import { AuthService } from "../auth/services/auth.service";

const addToken = (request: HttpRequest<unknown>, token: string): HttpRequest<unknown> => {
	return request.clone({
		setHeaders: {
			Authorization: `Bearer ${token}`,
		},
	});
};

const handle401Error = (
	request: HttpRequest<unknown>,
	next: HttpHandlerFn,
	authService: AuthService,
): Observable<HttpEvent<unknown>> => {
	if (!authService.isRefreshing) {
		return authService.refreshToken().pipe(
			switchMap((newAccessToken: string | null) => {
				if (newAccessToken) {
					return next(addToken(request, newAccessToken));
				}

				authService.logout();

				return throwError(() => new Error("Token refresh failed."));
			}),
			catchError((err) => {
				authService.logout();

				return throwError(() => err);
			}),
		);
	}

	return authService.refreshToken$.pipe(
		filter((token) => token !== null),
		take(1),
		switchMap((token) => {
			return next(addToken(request, token!));
		}),
	);
};

export const tokenInterceptor: HttpInterceptorFn = (
	req: HttpRequest<unknown>,
	next: HttpHandlerFn,
): Observable<HttpEvent<unknown>> => {
	const authService = inject(AuthService);

	const accessToken = authService.getAccessToken();
	let authReq = req;

	if (accessToken) {
		authReq = addToken(req, accessToken);
	}

	return next(authReq).pipe(
		catchError((error: HttpErrorResponse) => {
			if (error.status === 401 && !authReq.url.includes("/refresh") && accessToken) {
				return handle401Error(authReq, next, authService);
			}

			return throwError(() => error);
		}),
	);
};
