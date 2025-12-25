import { curry, nth, sum } from "ramda";

import type { ChartData, ChartOptions } from "chart.js";

import { env } from "@/app/shared/env";
import { getChartPalette } from "@/app/shared/ui/components/chart/lib/chart.util";

import type { CurrencyFormatter } from "../model/budgeting.model";

export const getMonthlyDonutChartConfig = (categories: { name: string; value: number }[]): ChartData<"doughnut"> => {
	let palette = getChartPalette();

	return {
		datasets: [
			{
				backgroundColor: palette,
				borderColor: "transparent",
				borderWidth: 2,
				data: categories.map((c) => c.value),
				hoverOffset: 15,
			},
		],
		labels: categories.map((c) => c.name),
	};
};

const isAbovePercent = curry((index: number, threshold: number, data: number[]) => {
	let value = nth(index, data) as number;
	let total = sum(data);

	return (value / total) * 100 > threshold;
});

export const getMonthlyDonutOptions = (currencyFormatter: CurrencyFormatter): ChartOptions<"doughnut"> => {
	let labelColor = env.getCssVariable("--text-primary");

	return {
		cutout: "70%",
		layout: {
			padding: 40,
		},
		maintainAspectRatio: false,
		plugins: {
			datalabels: {
				align: "end",
				anchor: "end",
				color: labelColor,
				display: (context) => {
					return isAbovePercent(context.dataIndex, 5, context.dataset.data as number[]);
				},
				font: {
					size: 12,
					weight: "bold",
				},
				formatter: (_, context) => {
					return context.chart.data.labels?.[context.dataIndex];
				},
				offset: 15,
			},
			legend: {
				display: false,
			},
			tooltip: {
				callbacks: {
					label: (context) => {
						let value = context.parsed;

						return ` ${context.label}: ${currencyFormatter(value)}`;
					},
				},
			},
		},
		responsive: true,
	};
};
