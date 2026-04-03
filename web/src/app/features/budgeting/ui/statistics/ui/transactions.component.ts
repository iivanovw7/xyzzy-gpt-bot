import type { StatisticsTransaction } from "@bindings";

import { config } from "@/app/shared/config";
import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ComboboxComponent from "@/app/shared/ui/components/combobox/combobox.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, effect, inject, signal, untracked } from "@angular/core";
import { toObservable, toSignal } from "@angular/core/rxjs-interop";
import { FormsModule } from "@angular/forms";
import { TuiTextfield } from "@taiga-ui/core";
import { debounceTime, distinctUntilChanged } from "rxjs";

import { StatisticsService } from "../service/statistics.service";

@Component({
	host: {
		class: "transactions",
	},
	imports: [CommonModule, FormsModule, ButtonComponent, SkeletonComponent, ComboboxComponent, TuiTextfield],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-transactions]",
	styleUrl: "./transactions.component.scss",
	templateUrl: "./transactions.component.html",
})
export default class TransactionsComponent {
	protected categoryFilterValue: Nullable<string> = null;

	protected readonly searchTerm = signal<string>("");
	protected readonly selectedCategory = signal<Nullable<string>>(null);
	protected readonly service = inject(StatisticsService);

	protected trackTransactionById = (_index: number, transaction: StatisticsTransaction) => {
		return transaction.id;
	};

	private readonly debouncedSearchTerm = toSignal(
		toObservable(this.searchTerm).pipe(debounceTime(config.ui.debounce), distinctUntilChanged()),
		{ initialValue: "" },
	);

	constructor() {
		effect(() => {
			let category = this.selectedCategory();
			let description = this.debouncedSearchTerm() || null;

			untracked(() => {
				this.service.queryTransactions({ category, description });
			});
		});
	}
}
