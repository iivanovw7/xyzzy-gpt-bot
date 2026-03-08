import { descend, prop, sort, splitAt, sum } from "ramda";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import ExpandComponent from "@/app/shared/ui/components/expand/expand.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule, CurrencyPipe } from "@angular/common";
import { Component, computed, inject, input } from "@angular/core";
import { pipe } from "rxjs";

import { OverviewService } from "../service/overview.service";

@Component({
	host: {
		class: "categories-ranking",
	},
	imports: [CommonModule, ButtonComponent, ExpandComponent, SkeletonComponent],
	providers: [CurrencyPipe],
	selector: "div[app-budgeting-categories-ranking]",
	styleUrl: "./categories-ranking.component.scss",
	templateUrl: "./categories-ranking.component.html",
})
export default class CategoriesRankingComponent {
	protected categoriesIncomeRankingExpanded = false;
	protected categoriesSpendingRankingExpanded = false;

	protected readonly service = inject(OverviewService);

	protected currentMonthIndex = computed(() => {
		return this.service.overview()?.month ?? 1;
	});

	protected monthlyIncomeCategoriesRanking = computed(() => {
		let overview = this.service.overview();
		let summaries = overview?.yearSummary.monthly_income_summaries;

		return this.getRanking(summaries, this.currentMonthIndex() - 1);
	});

	protected initialIncomeCategoriesRanking = computed(() => {
		return splitAt(5, this.monthlyIncomeCategoriesRanking()).at(0);
	});

	protected monthlySpendingCategoriesRanking = computed(() => {
		let overview = this.service.overview();
		let summaries = overview?.yearSummary.monthly_spending_summaries;

		return this.getRanking(summaries, this.currentMonthIndex() - 1);
	});

	protected initialSpendingCategoriesRanking = computed(() => {
		return splitAt(5, this.monthlySpendingCategoriesRanking()).at(0);
	});

	protected restIncomeCategoriesRanking = computed(() => {
		return splitAt(5, this.monthlyIncomeCategoriesRanking()).at(1);
	});

	protected restSpendingCategoriesRanking = computed(() => {
		return splitAt(5, this.monthlySpendingCategoriesRanking()).at(1);
	});

	subtitle = input.required<string>();

	private getRanking<T extends { amounts: number[]; name: string }>(summaries: T[] | undefined, monthIndex: number) {
		if (!summaries) return [];

		let sortedCategories = pipe(
			(items: T[]) => {
				return items.map((s) => ({
					name: s.name,
					value: s.amounts[monthIndex] || 0,
				}));
			},
			(items) => items.filter((index) => index.value > 0),
			sort(descend(prop("value"))),
			(sortedItems) => {
				let total = sum(sortedItems.map(prop("value")));

				return sortedItems.map((item) => ({
					...item,
					percentage: total > 0 ? (item.value / total) * 100 : 0,
				}));
			},
		);

		return sortedCategories(summaries);
	}
}
