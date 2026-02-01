import type { ChartData, ChartOptions, TooltipItem } from "chart.js";
import type { Context } from "chartjs-plugin-datalabels";

import type { CategorySummary } from "@bindings";

import { env } from "@/app/shared/env";

import type { CurrencyFormatter } from "../model/budgeting.model";

import { MONTH_LABELS } from "../model/budgeting.model";

export const getCategoriesOverveiwConfig = (category: CategorySummary): ChartData<"bar"> => {
	let incomeColor = env.getCssVariable("--text-success");
	let spendingColor = env.getCssVariable("--text-error");
	let neutralColor = env.getCssVariable("--divider-dark");

	let data = Array(12).fill(0);

	for (let summary of category.monthlySummaries) {
		let monthIndex = summary.month - 1;
		data[monthIndex] = summary.income - summary.spending;
	}

	return {
		datasets: [
			{
				backgroundColor: (context) => {
					let value = context.raw as number;

					if (value > 0) return incomeColor;
					if (value < 0) return spendingColor;

					return neutralColor;
				},
				borderRadius: 0,
				borderSkipped: false,
				data,
				label: category.category,
				stack: "category",
			},
		],
		labels: MONTH_LABELS,
	};
};

export const getCategoriesOverviewOptions = (currencyFormatter: CurrencyFormatter): ChartOptions<"bar"> => {
	let labelColor = env.getCssVariable("--text-primary");
	let gridColor = env.getCssVariable("--divider-dark");

	let formatSigned = (value: number) => `${value > 0 ? "+" : ""}${currencyFormatter(value)}`;

	return {
		layout: {
			padding: {
				top: 40,
			},
		},
		maintainAspectRatio: false,
		plugins: {
			datalabels: {
				align: (context: Context) => {
					let value = context.dataset.data[context.dataIndex] as number;

					return value >= 0 ? "end" : "start";
				},
				anchor: (context: Context) => {
					let value = context.dataset.data[context.dataIndex] as number;

					return value >= 0 ? "end" : "start";
				},
				color: labelColor,
				font: {
					family: "monospace",
					size: 10,
				},
				formatter: (value: number) => (value === 0 ? "" : formatSigned(value)),
				offset: 5,
				rotation: -90,
				textAlign: "center",
			},
			legend: {
				display: false,
			},
			tooltip: {
				callbacks: {
					label: (context: TooltipItem<"bar">) => {
						let value = context.parsed.y;

						return `${context.dataset.label}: ${formatSigned(value || 0)}`;
					},
				},
				intersect: false,
				mode: "index",
				padding: 12,
			},
		},
		responsive: true,
		scales: {
			x: {
				grid: { color: gridColor },
				ticks: { color: labelColor },
			},
			y: {
				beginAtZero: true,
				border: { display: false },
				grid: { color: gridColor },
				ticks: {
					callback: (value) => (typeof value === "number" ? formatSigned(value) : value),
					color: labelColor,
					maxTicksLimit: 5,
				},
			},
		},
	};
};
