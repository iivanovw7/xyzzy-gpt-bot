import type { HttpErrorResponse, HttpInterceptorFn } from "@angular/common/http";

import { inject } from "@angular/core";
import { throwError } from "rxjs";
import { catchError } from "rxjs/operators";

import { NotificationService } from "../../shared/ui/components/notification/notification.service";

export const errorInterceptor: HttpInterceptorFn = (request, next) => {
	let notificationService = inject(NotificationService);

	return next(request).pipe(
		catchError((error: HttpErrorResponse) => {
			let message = error.error?.message ?? error.message ?? "Something went wrong";

			notificationService.error(message, { label: "Error" });

			return throwError(() => error);
		}),
	);
};
