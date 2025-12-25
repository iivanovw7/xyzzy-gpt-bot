import type { ChartData, ChartOptions } from "chart.js";

import { env } from "@/app/shared/env";
import { getChartPalette } from "@/app/shared/ui/components/chart/lib/chart.util";

import type { CurrencyFormatter } from "../model/budgeting.model";

import { MONTH_LABELS } from "../model/budgeting.model";
import { getYearlyBarChartOptions } from "./yearly-overview.util";

export const getCategoryStackedChartConfig = (categories: { data: number[]; name: string }[]): ChartData<"bar"> => {
	let palette = getChartPalette();

	return {
		datasets: categories.map((category, index) => ({
			backgroundColor: palette[index % palette.length] || palette[0],
			borderRadius: 0,
			borderSkipped: false,
			data: category.data,
			label: category.name,
			stack: "Stack 0",
		})),
		labels: MONTH_LABELS,
	};
};

export const getCategoryStackedOptions = (currencyFormatter: CurrencyFormatter): ChartOptions<"bar"> => {
	let options = getYearlyBarChartOptions(currencyFormatter);
	let labelColor = env.getCssVariable("--text-primary");
	let gridColor = env.getCssVariable("--divider-dark");

	return {
		...options,
		plugins: {
			...options.plugins,
			datalabels: {
				display: false,
			},
			legend: {
				...options.plugins?.legend,
				labels: {
					...options.plugins?.legend?.labels,
					boxWidth: 12,
					color: labelColor,
					padding: 15,
				},
				position: "bottom",
			},
		},
		scales: {
			x: {
				grid: { color: gridColor },
				stacked: true,
				ticks: { color: labelColor },
			},
			y: {
				beginAtZero: true,
				grid: { color: gridColor },
				stacked: true,
				ticks: { color: labelColor },
			},
		},
	};
};
