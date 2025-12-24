import { pluck } from "ramda";

import type { ChartData, ChartOptions, TooltipItem } from "chart.js";

import type { OverviewResponse } from "@bindings";

import { env } from "@/app/shared/env";

import type { CurrencyFormatter } from "../model/overview.model";

import { MONTH_LABELS } from "../model/overview.model";

export const getYearlyBarChartConfig = (summary: OverviewResponse["yearSummary"]): ChartData<"bar"> => {
	let incomeColor = env.getCssVariable("--text-success");
	let spendingColor = env.getCssVariable("--text-error");

	return {
		datasets: [
			{
				backgroundColor: incomeColor,
				borderRadius: 0,
				borderSkipped: false,
				data: pluck("income", summary.monthly_summaries),
				label: "Income",
			},
			{
				backgroundColor: spendingColor,
				borderRadius: 0,
				borderSkipped: false,
				data: pluck("spending", summary.monthly_summaries),
				label: "Spending",
			},
		],
		labels: MONTH_LABELS,
	};
};

export const getYearlyBarChartOptions = (currencyFormatter: CurrencyFormatter): ChartOptions<"bar"> => {
	let labelColor = env.getCssVariable("--text-primary");
	let gridColor = env.getCssVariable("--divider-dark");

	let dateLabelFormatter = (value: number) => (value > 0 ? currencyFormatter(value) : "");

	let tooltipLabelFormatter = (context: TooltipItem<"bar">) => {
		let label = context.dataset.label || "";

		if (context.parsed.y !== null) {
			label += `: ${currencyFormatter(context.parsed.y)}`;
		}

		return label;
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
				formatter: dateLabelFormatter,
				offset: 5,
				rotation: -90,
				textAlign: "center",
			},
			legend: {
				align: "center",
				display: true,
				labels: {
					color: labelColor,
					pointStyle: "rect",
					usePointStyle: true,
				},
				position: "bottom",
			},
			tooltip: {
				callbacks: {
					label: tooltipLabelFormatter,
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
