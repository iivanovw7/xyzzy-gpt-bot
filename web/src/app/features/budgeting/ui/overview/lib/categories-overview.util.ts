import type { ChartData, ChartOptions } from "chart.js";

import type { CategorySummary } from "@bindings";

import { env } from "@/app/shared/env";

import type { CurrencyFormatter } from "../model/budgeting.model";

import { MONTH_LABELS } from "../model/budgeting.model";

export const getCategoriesOverveiwConfig = (category: CategorySummary): ChartData<"bar"> => {
	let incomeColor = env.getCssVariable("--text-success");
	let spendingColor = env.getCssVariable("--text-error");
	let neutralColor = env.getCssVariable("--divider-dark");

	let data = Array(12).fill(0);
	let colors = Array(12).fill(neutralColor);

	for (let summary of category.monthlySummaries) {
		let monthIndex = summary.month - 1;

		if (summary.income) {
			data[monthIndex] = summary.income;
			colors[monthIndex] = incomeColor;
		} else if (summary.spending > 0) {
			data[monthIndex] = summary.spending;
			colors[monthIndex] = spendingColor;
		}
	}

	return {
		datasets: [
			{
				backgroundColor: colors,
				borderRadius: 0,
				borderSkipped: false,
				data,
				label: category.category,
			},
		],
		labels: MONTH_LABELS,
	};
};

export const getCategoriesOverviewOptions = (currencyFormatter: CurrencyFormatter): ChartOptions<"bar"> => {
	let labelColor = env.getCssVariable("--text-primary");
	let gridColor = env.getCssVariable("--divider-dark");

	let format = (value: number) => currencyFormatter(value);

	return {
		layout: {
			padding: {
				top: 50,
			},
		},
		maintainAspectRatio: false,
		plugins: {
			datalabels: {
				align: "end",
				anchor: "end",
				color: labelColor,
				font: {
					family: "monospace",
					size: 10,
				},
				formatter: (value: number) => (value === 0 ? "" : format(value)),
				offset: 6,
				rotation: -90,
				textAlign: "center",
			},
			legend: {
				display: false,
			},
			tooltip: {
				callbacks: {
					label: (context) => `${context.dataset.label}: ${format(context.parsed.y || 0)}`,
				},
			},
		},
		scales: {
			x: {
				grid: { color: gridColor },
				offset: true,
				ticks: { color: labelColor },
			},
			y: {
				beginAtZero: true,
				border: { display: false },
				grid: { color: gridColor },
				ticks: {
					color: labelColor,
					maxTicksLimit: 5,
				},
			},
		},
	};
};
