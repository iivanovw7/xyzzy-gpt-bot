import { splitAt } from "ramda";

import type { OnInit } from "@angular/core";
import type { OverviewTransaction } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ChartComponent from "@/app/shared/ui/components/chart/chart.component";
import ExpandComponent from "@/app/shared/ui/components/expand/expand.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, inject } from "@angular/core";

import { getMonthlyDonutChartConfig, getMonthlyDonutOptions } from "../../lib/monthly-breakdown.util";
import { getYearlyBarChartConfig, getYearlyBarChartOptions } from "../../lib/yearly-overview.util";
import { getCategoryStackedChartConfig, getCategoryStackedOptions } from "../../lib/yearly-trends.util";
import { BudgetingService } from "../../service";

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
	protected readonly service = inject(BudgetingService);

	protected currentMonthIndex = computed(() => {
		let overview = this.service.overview();

		return overview ? overview.month : 1;
	});

	protected monthlyBreakdownData = computed(() => {
		let overview = this.service.overview();

		if (!overview?.yearSummary.monthly_spending_summaries) return null;

		let categoryData = overview.yearSummary.monthly_spending_summaries.map((category) => ({
			name: category.name,
			value: category.amounts[this.currentMonthIndex() - 1],
		}));

		return getMonthlyDonutChartConfig(categoryData);
	});

	private currencyPipe = inject(CurrencyPipe);

	protected monthlyBreakdownOptions = computed(() => {
		let currencyFormatter = (value: number): string => {
			return this.currencyPipe.transform(value, this.service.overview()?.currency, "symbol", "1.2-2") ?? "";
		};

		return getMonthlyDonutOptions(currencyFormatter);
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

	protected yearlyOverviewData = computed(() => {
		let summary = this.service.overview()?.yearSummary;

		return summary ? getYearlyBarChartConfig(summary) : null;
	});

	protected yearlyOverviewDataYear = computed(() => {
		return this.service.overview()?.yearSummary.year;
	});

	protected yearlyOverviewOptions = computed(() => {
		let currencyFormatter = (value: number): string => {
			return this.currencyPipe.transform(value, this.service.overview()?.currency, "symbol", "1.2-2") ?? "";
		};

		return getYearlyBarChartOptions(currencyFormatter);
	});

	protected yearlyTrendsData = computed(() => {
		let overview = this.service.overview();

		if (!overview?.yearSummary.monthly_spending_summaries) return null;

		return getCategoryStackedChartConfig(
			overview.yearSummary.monthly_spending_summaries.map((c) => ({
				data: c.amounts,
				name: c.name,
			})),
		);
	});

	protected yearlyTrendsOptions = computed(() => {
		let currencyFormatter = (value: number): string => {
			let formatted =
				this.currencyPipe.transform(value, this.service.overview()?.currency, "symbol", "1.2-2") ?? "";

			return `- ${formatted}`;
		};

		return getCategoryStackedOptions(currencyFormatter);
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
