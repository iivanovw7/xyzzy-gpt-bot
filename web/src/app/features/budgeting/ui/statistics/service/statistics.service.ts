import { groupBy, mapObjIndexed } from "ramda";

import type { StatisticsTransaction, TransactionQuery, TransactionsResponse } from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient, HttpParams } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class StatisticsService {
	private getMonthFromSeconds = (transaction: StatisticsTransaction) => {
		return String(new Date(transaction.date * 1000).getMonth());
	};

	private http = inject(HttpClient);

	transactions = signal<Nullable<TransactionsResponse>>(null);

	accumulatedAmmountsMonthly = computed<Record<string, number>>(() => {
		let transactions = this.transactions()?.transactions;

		if (!transactions) return {};

		let grouped = groupBy(this.getMonthFromSeconds, transactions);
		let accumulatedAmounts = (list: StatisticsTransaction[]) => {
			return list.reduce(
				(total, transaction) => total + (transaction.isIncome ? transaction.amount : -transaction.amount),
				0,
			);
		};

		return mapObjIndexed(accumulatedAmounts, grouped) as Record<string, number>;
	});

	error = signal<boolean>(false);

	getAccumulatedAmount = (transaction: StatisticsTransaction) => {
		return this.accumulatedAmmountsMonthly()[this.getMonthFromSeconds(transaction)];
	};

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
