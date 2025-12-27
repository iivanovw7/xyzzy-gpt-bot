import type { TransactionQuery, TransactionsResponse } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient, HttpParams } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class StatisticsService {
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
			case !!this.transactions(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	transactions = signal<Nullable<TransactionsResponse>>(null);

	queryTransactions(filters: TransactionQuery) {
		this.isLoading.set(true);
		this.error.set(false);

		let parameters = new HttpParams();

		if (filters.category) {
			parameters = parameters.set("category", filters.category);
		}
		if (filters.description) {
			parameters = parameters.set("description", filters.description);
		}

		this.http
			.get<QueryResponse<TransactionsResponse>>("/budgeting/transactions", { params: parameters })
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("TransactionsService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.transactions.set(response?.data ?? null);
			});
	}
}
