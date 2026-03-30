import type { CreateTransactionRequest, CreateTransactionResponse, RecurrentDashboard } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class RecurrentService {
	private http = inject(HttpClient);

	dashboard = signal<Nullable<RecurrentDashboard>>(null);
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
			case !!this.dashboard(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	postTransaction(data: CreateTransactionRequest, onSuccess: () => void, onComplete?: () => void) {
		this.http
			.post<QueryResponse<CreateTransactionResponse>>("/budgeting/transactions", data)
			.pipe(
				finalize(() => {
					if (onComplete) onComplete();
				}),
				catchError((errorData) => {
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
			.get<QueryResponse<RecurrentDashboard>>("/budgeting/recurrent")
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("RecurrentService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.dashboard.set(response?.data ?? null);
			});
	}
}
