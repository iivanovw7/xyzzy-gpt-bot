import { splitAt } from "ramda";

import type { OnInit } from "@angular/core";
import type { OverviewTransaction } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ChartComponent from "@/app/shared/ui/components/chart/chart.component";
import ExpandComponent from "@/app/shared/ui/components/expand/expand.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, inject } from "@angular/core";

import { getYearlyBarChartConfig, getYearlyBarChartOptions } from "../../lib/overview.chart";
import { OverviewService } from "../../service";

@Component({
	host: {
		class: "budgeting__overview",
	},
	imports: [CommonModule, ButtonComponent, ExpandComponent, SkeletonComponent, ChartComponent],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-overview]",
	styleUrl: "./overview.component.scss",
	templateUrl: "./overview.component.html",
})
export default class BudgetingOverveiwComponent implements OnInit {
	protected readonly service = inject(OverviewService);
	private currencyPipe = inject(CurrencyPipe);

	protected barOptions = computed(() => {
		let currencyFormatter = (value: number): string => {
			return this.currencyPipe.transform(value, this.service.overview()?.currency, "symbol", "1.2-2") ?? "";
		};

		return getYearlyBarChartOptions(currencyFormatter);
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

	protected status = computed(() => {
		switch (true) {
			case this.service.isLoading(): {
				return "loading";
			}
			case this.service.error(): {
				return "error";
			}
			case !!this.service.overview(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	protected trackTransactionById = (_index: number, transaction: OverviewTransaction) => {
		return transaction.id;
	};

	protected transactionsExpanded = false;

	protected yearlyData = computed(() => {
		let summary = this.service.overview()?.yearSummary;

		return summary ? getYearlyBarChartConfig(summary) : null;
	});

	ngOnInit() {
		this.service.query();
	}

	transformMonth(monthNumber: number): string {
		let date = new Date();

		date.setMonth(monthNumber - 1);

		return date.toLocaleString("default", { month: "long" });
	}
}
