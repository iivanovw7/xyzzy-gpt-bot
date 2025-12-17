import type { HttpErrorResponse, HttpInterceptorFn } from "@angular/common/http";

import { throwError } from "rxjs";
import { catchError } from "rxjs/operators";

export const errorInterceptor: HttpInterceptorFn = (request, next) => {
	return next(request).pipe(
		catchError((error: HttpErrorResponse) => {
			return throwError(() => error);
		}),
	);
};
