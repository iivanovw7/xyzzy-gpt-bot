import type { ChartData, ChartOptions } from "chart.js";

import { env } from "@/app/shared/env";

import type { CurrencyFormatter } from "../model/overview.model";

import { MONTH_LABELS } from "../model/overview.model";
import { getYearlyBarChartOptions } from "./yearly-overview.util";

export const getCategoryStackedChartConfig = (categories: { data: number[]; name: string }[]): ChartData<"bar"> => {
	let palette = [
		env.getCssVariable("--accent1"),
		env.getCssVariable("--accent2"),
		env.getCssVariable("--accent3"),
		env.getCssVariable("--accent4"),
		env.getCssVariable("--accent5"),
		env.getCssVariable("--accent6"),
	];

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
					padding: 15,
				},
				position: "bottom",
			},
		},
		scales: {
			x: {
				grid: { color: gridColor },
				stacked: true,
			},
			y: {
				beginAtZero: true,
				grid: { color: gridColor },
				stacked: true,
			},
		},
	};
};
