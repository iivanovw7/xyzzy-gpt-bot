import type { OnInit } from "@angular/core";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ChartComponent from "@/app/shared/ui/components/chart/chart.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, inject } from "@angular/core";

import { BudgetingService } from "../../service";
import { getMonthlyDonutChartConfig, getMonthlyDonutOptions } from "./lib/monthly-breakdown.util";
import { getYearlyBarChartConfig, getYearlyBarChartOptions } from "./lib/yearly-overview.util";
import { getCategoryStackedChartConfig, getCategoryStackedOptions } from "./lib/yearly-trends.util";
import CategoriesRankingComponent from "./ui/categories-ranking.component";
import TransactionsComponent from "./ui/transactions.component";

@Component({
	host: {
		class: "budgeting__overview",
	},
	imports: [
		CommonModule,
		ButtonComponent,
		SkeletonComponent,
		ChartComponent,
		CategoriesRankingComponent,
		TransactionsComponent,
	],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-overview]",
	styleUrl: "./overview.component.scss",
	templateUrl: "./overview.component.html",
})
export default class BudgetingOverveiwComponent implements OnInit {
	protected categoriesRankingExpanded = false;

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
