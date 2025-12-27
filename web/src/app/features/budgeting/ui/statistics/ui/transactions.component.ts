import type { BudgetingTransaction } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ComboboxComponent from "@/app/shared/ui/components/combobox/combobox.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, effect, inject, signal, untracked } from "@angular/core";

import { StatisticsService } from "../service/statistics.service";

@Component({
	host: {
		class: "transactions",
	},
	imports: [CommonModule, ButtonComponent, SkeletonComponent, ComboboxComponent],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-transactions]",
	styleUrl: "./transactions.component.scss",
	templateUrl: "./transactions.component.html",
})
export default class TransactionsComponent {
	protected categoryFilterValue: Nullable<string> = null;

	protected readonly selectedCategory = signal<null | string>(null);
	protected readonly service = inject(StatisticsService);

	protected trackTransactionById = (_index: number, transaction: BudgetingTransaction) => {
		return transaction.id;
	};

	constructor() {
		effect(() => {
			let category = this.selectedCategory();

			untracked(() => {
				this.service.queryTransactions({ category, description: null });
			});
		});
	}
}
