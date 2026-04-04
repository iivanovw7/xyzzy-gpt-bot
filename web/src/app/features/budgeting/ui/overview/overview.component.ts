import type { OnInit } from "@angular/core";

import { allKeyValuesZero } from "@/app/shared/list";
import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ChartComponent from "@/app/shared/ui/components/chart/chart.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, effect, inject, signal } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { TuiButtonX, TuiDropdown, TuiFilterByInputPipe, TuiInput, TuiTextfield } from "@taiga-ui/core";
import { TuiComboBox, TuiDataListWrapper } from "@taiga-ui/kit";

import { getCategoriesOverveiwConfig, getCategoriesOverviewOptions } from "./lib/categories-overview.util";
import { getMonthlyDonutChartConfig, getMonthlyDonutOptions } from "./lib/monthly-breakdown.util";
import { getYearlyBarChartConfig, getYearlyBarChartOptions } from "./lib/yearly-overview.util";
import { getCategoryStackedChartConfig, getCategoryStackedOptions } from "./lib/yearly-trends.util";
import { OverviewService } from "./service/overview.service";
import CategoriesRankingComponent from "./ui/categories-ranking.component";

@Component({
	host: {
		class: "budgeting__overview",
	},
	imports: [
		CommonModule,
		FormsModule,
		ButtonComponent,
		SkeletonComponent,
		ChartComponent,
		CategoriesRankingComponent,
		TuiComboBox,
		TuiDataListWrapper,
		TuiFilterByInputPipe,
		TuiDropdown,
		TuiTextfield,
		TuiButtonX,
		TuiInput,
	],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-overview]",
	styleUrl: "./overview.component.scss",
	templateUrl: "./overview.component.html",
})
export default class BudgetingOverveiwComponent implements OnInit {
	protected readonly service = inject(OverviewService);

	protected categoriesList = computed(() => {
		let overview = this.service.overview();

		if (!overview?.categoriesSummary.categories) return [];

		return overview.categoriesSummary.categories.map(({ category }) => category);
	});

	protected categoriesOverviewCategories = computed(() => {
		return this.service.overview()?.categoriesSummary.categories;
	});

	protected readonly selectedOverviewCategory = signal<Nullable<string>>(null);

	protected categoriesOverviewData = computed(() => {
		let overview = this.service.overview();
		let categories = overview?.categoriesSummary.categories;

		if (!categories) return null;

		let category = categories.find((value) => {
			return value.category === this.selectedOverviewCategory();
		});

		if (!category) return null;

		return getCategoriesOverveiwConfig(category);
	});

	private currencyFormatter = (value: number): string => {
		let currency = this.service.overview()?.currency;

		return this.currencyPipe.transform(value, currency, "symbol", "1.2-2") ?? "";
	};

	protected categoriesOverviewOptions = computed(() => {
		return getCategoriesOverviewOptions(this.currencyFormatter);
	});

	protected categoriesOverviewYear = computed(() => {
		return this.service.overview()?.categoriesSummary.year;
	});

	protected categoriesRankingExpanded = false;

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

		if (allKeyValuesZero(categoryData, "value")) return null;

		return getMonthlyDonutChartConfig(categoryData);
	});

	protected monthlyBreakdownOptions = computed(() => {
		return getMonthlyDonutOptions(this.currencyFormatter);
	});

	protected yearlyIncomeTrendsData = computed(() => {
		let overview = this.service.overview();

		if (!overview?.yearSummary.monthly_income_summaries) return null;

		return getCategoryStackedChartConfig(
			overview.yearSummary.monthly_income_summaries.map((category) => ({
				data: category.amounts,
				name: category.name,
			})),
		);
	});

	protected yearlyIncomeTrendsOptions = computed(() => {
		return getCategoryStackedOptions(this.currencyFormatter);
	});

	protected yearlyOverviewData = computed(() => {
		let summary = this.service.overview()?.yearSummary;

		return summary ? getYearlyBarChartConfig(summary) : null;
	});

	protected yearlyOverviewDataYear = computed(() => {
		return this.service.overview()?.yearSummary.year;
	});

	protected yearlyOverviewOptions = computed(() => {
		return getYearlyBarChartOptions(this.currencyFormatter);
	});

	protected yearlySpendingTrendsData = computed(() => {
		let overview = this.service.overview();

		if (!overview?.yearSummary.monthly_spending_summaries) return null;

		return getCategoryStackedChartConfig(
			overview.yearSummary.monthly_spending_summaries.map((category) => ({
				data: category.amounts,
				name: category.name,
			})),
		);
	});

	protected yearlySpendingTrendsOptions = computed(() => {
		return getCategoryStackedOptions((value) => `- ${this.currencyFormatter(value)}`);
	});

	private currencyPipe = inject(CurrencyPipe);

	constructor() {
		effect(() => {
			let categories = this.service.overview()?.categoriesSummary.categories;

			if (!categories?.length) return;

			if (!this.selectedOverviewCategory()) {
				this.selectedOverviewCategory.set(categories[0].category);
			}
		});
	}

	ngOnInit() {
		this.service.queryOverview();
	}

	transformMonth(monthNumber: number): string {
		let date = new Date();

		date.setMonth(monthNumber - 1);

		return date.toLocaleString("default", { month: "long" });
	}
}
