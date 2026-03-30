import type { HttpHandlerFn, HttpInterceptorFn, HttpRequest } from "@angular/common/http";

import { inject } from "@angular/core";
import { finalize } from "rxjs";

import { LoadingService } from "../services/loading.service";

export const loadingInterceptor: HttpInterceptorFn = (request: HttpRequest<unknown>, next: HttpHandlerFn) => {
	let loadingService = inject(LoadingService);

	loadingService.setLoading(true);

	return next(request).pipe(finalize(() => loadingService.setLoading(false)));
};
