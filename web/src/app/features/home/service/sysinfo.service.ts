import type { SysInfoResponse } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class SysInfoService {
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
			case !!this.sysInfo(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	sysInfo = signal<Nullable<SysInfoResponse>>(null);

	querySysInfo() {
		this.isLoading.set(true);
		this.error.set(false);

		this.http
			.get<QueryResponse<SysInfoResponse>>("/sysinfo")
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("OverviewService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.sysInfo.set(response?.data ?? null);
			});
	}
}
