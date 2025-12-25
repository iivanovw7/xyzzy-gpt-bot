import type { OverviewResponse } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class BudgetingService {
	private http = inject(HttpClient);

	error = signal<boolean>(false);
	isLoading = signal<boolean>(false);
	overview = signal<Nullable<OverviewResponse>>(null);

	status = computed(() => {
		switch (true) {
			case this.isLoading(): {
				return "loading";
			}
			case this.error(): {
				return "error";
			}
			case !!this.overview(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	query() {
		this.isLoading.set(true);
		this.error.set(false);

		this.http
			.get<QueryResponse<OverviewResponse>>("/budgeting/overview")
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("OverviewService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.overview.set(response?.data ?? null);
			});
	}
}
