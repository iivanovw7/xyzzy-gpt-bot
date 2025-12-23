import { splitAt } from "ramda";

import type { OnInit } from "@angular/core";
import type { OverviewTransaction } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ExpandComponent from "@/app/shared/ui/components/expand/expand.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, inject } from "@angular/core";

import { OverviewService } from "../../service";

@Component({
	host: {
		class: "budgeting__overview",
	},
	imports: [CurrencyPipe, CommonModule, ButtonComponent, ExpandComponent, SkeletonComponent],
	selector: "div[app-budgeting-overview]",
	styleUrl: "./overview.component.scss",
	templateUrl: "./overview.component.html",
})
export default class BudgetingOverveiwComponent implements OnInit {
	protected readonly service = inject(OverviewService);

	private allTransactions = computed(() => {
		return this.service.overview()?.monthTransactions ?? [];
	});

	protected recentTransactions = computed(() => {
		return splitAt(5, this.allTransactions()).at(0);
	});

	protected restTransactions = computed(() => {
		return splitAt(5, this.allTransactions()).at(1);
	});

	protected status = computed(() => {
		if (this.service.isLoading()) return "loading";
		if (this.service.error()) return "error";
		if (this.service.overview()) return "success";

		return "idle";
	});

	protected trackTransactionById = (_inx: number, transaction: OverviewTransaction) => {
		return transaction.id;
	};

	protected transactionsExpanded = false;

	ngOnInit() {
		this.service.query();
	}

	transformMonth(monthNumber: number): string {
		let date = new Date();

		date.setMonth(monthNumber - 1);

		return date.toLocaleString("default", { month: "long" });
	}
}
