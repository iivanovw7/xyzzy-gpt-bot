import type { HttpInterceptorFn } from "@angular/common/http";

import { environment } from "../../../../environments/environment";

export const apiInterceptor: HttpInterceptorFn = (request, next) => {
	let apiUrl = environment.API_BASE_URL;

	let apiRequest = request.clone({
		url: `${apiUrl}/api${request.url}`,
	});

	return next(apiRequest);
};
