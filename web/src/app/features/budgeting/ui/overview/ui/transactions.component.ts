import { splitAt } from "ramda";

import type { OverviewTransaction } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ExpandComponent from "@/app/shared/ui/components/expand/expand.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, inject } from "@angular/core";

import { BudgetingService } from "../../../service";

@Component({
	host: {
		class: "transactions",
	},
	imports: [CommonModule, ButtonComponent, ExpandComponent, SkeletonComponent],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-transactions]",
	styleUrl: "./transactions.component.scss",
	templateUrl: "./transactions.component.html",
})
export default class TransactionsComponent {
	protected readonly service = inject(BudgetingService);

	protected currentMonthIndex = computed(() => {
		return this.service.overview()?.month ?? 1;
	});

	private allTransactions = computed(() => {
		return this.service.overview()?.monthTransactions ?? [];
	});

	protected recentTransactions = computed(() => {
		return splitAt(5, this.allTransactions()).at(0);
	});

	protected restTransactions = computed(() => {
		return splitAt(5, this.allTransactions()).at(1);
	});

	protected trackTransactionById = (_index: number, transaction: OverviewTransaction) => {
		return transaction.id;
	};

	protected transactionsExpanded = false;
}
