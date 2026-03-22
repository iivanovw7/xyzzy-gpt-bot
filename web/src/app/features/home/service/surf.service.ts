import type { SurfReportResponse } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class SurfService {
	private http = inject(HttpClient);

	error = signal<boolean>(false);
	isLoading = signal<boolean>(false);

	status = computed(() => {
		switch (true) {
			case this.isLoading(): {
				return "loading";
			}
			case this.error(): {
				return "error";
			}
			case !!this.surfReport(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	surfReport = signal<Nullable<SurfReportResponse>>(null);

	querySurfReport() {
		this.isLoading.set(true);
		this.error.set(false);

		this.http
			.get<QueryResponse<SurfReportResponse>>("/surf/forecast")
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("SurfService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.surfReport.set(response?.data ?? null);
			});
	}
}
