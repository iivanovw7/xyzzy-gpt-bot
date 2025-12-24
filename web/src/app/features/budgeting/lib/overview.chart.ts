import { pluck } from "ramda";

import type { ChartData, ChartOptions } from "chart.js";

import type { OverviewResponse } from "@bindings";

import { env } from "@/app/shared/env";

export const getYearlyBarChartConfig = (summary: OverviewResponse["yearSummary"]): ChartData<"bar"> => {
	let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
	let incomeColor = env.getCssVariable("--text-success");
	let spendingColor = env.getCssVariable("--text-error");

	return {
		datasets: [
			{
				backgroundColor: incomeColor,
				borderRadius: 0,
				borderSkipped: false,
				data: pluck("income", summary.summaries),
				label: "Income",
			},
			{
				backgroundColor: spendingColor,
				borderRadius: 0,
				borderSkipped: false,
				data: pluck("spending", summary.summaries),
				label: "Spending",
			},
		],
		labels: months,
	};
};

export const getYearlyBarChartOptions = (
	currencyFormatter: (value: number) => Nullable<string>,
): ChartOptions<"bar"> => {
	let labelColor = env.getCssVariable("--text-primary");
	let gridColor = env.getCssVariable("--divider-dark");

	let formatter = (value: number) => {
		return value > 0 ? currencyFormatter(value) || "" : "";
	};

	return {
		layout: {
			padding: {
				top: 40,
			},
		},
		maintainAspectRatio: false,
		plugins: {
			datalabels: {
				align: -90,
				anchor: "end",
				color: labelColor,
				font: {
					family: "monospace",
					size: 10,
				},
				formatter,
				offset: 5,
				rotation: -90,
				textAlign: "center",
			},
			legend: {
				align: "end",
				display: true,
				labels: {
					color: labelColor,
					pointStyle: "rect",
					usePointStyle: true,
				},
				position: "chartArea",
			},
			tooltip: {
				intersect: false,
				mode: "index",
				padding: 12,
			},
		},
		responsive: true,
		scales: {
			x: {
				grid: { display: false },
				stacked: false,
				ticks: { color: labelColor },
			},
			y: {
				beginAtZero: true,
				border: { display: false },
				grid: { color: gridColor },
				ticks: { color: labelColor, maxTicksLimit: 5 },
			},
		},
	};
};
