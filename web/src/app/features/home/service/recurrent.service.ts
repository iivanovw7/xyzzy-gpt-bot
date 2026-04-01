import type { CreateTransactionRequest, CreateTransactionResponse, RecurrentDashboards } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class RecurrentService {
	private http = inject(HttpClient);

	dashboards = signal<Nullable<RecurrentDashboards>>(null);
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
			case !!this.dashboards(): {
				return "success";
			}
			default: {
				return "loading";
			}
		}
	});

	postTransaction(request: CreateTransactionRequest, onSuccess: () => void, onError: () => void) {
		this.http
			.post<QueryResponse<CreateTransactionResponse>>("/budgeting/transactions", request)
			.pipe(
				catchError((errorData) => {
					onError();
					logger.error("RecurrentService post error", errorData.message);

					return of(null);
				}),
			)
			.subscribe((response) => {
				if (response?.data.success) {
					onSuccess();
					this.queryRecurrent();
				}
			});
	}

	queryRecurrent() {
		this.isLoading.set(true);
		this.error.set(false);

		this.http
			.get<QueryResponse<RecurrentDashboards>>("/budgeting/recurrent")
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("RecurrentService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.dashboards.set(response?.data ?? null);
			});
	}
}
