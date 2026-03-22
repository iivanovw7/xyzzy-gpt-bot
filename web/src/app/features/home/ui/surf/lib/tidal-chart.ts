import type { ChartConfiguration } from "chart.js";

import { env } from "@/app/shared/env";

export const getTideChartConfig = (tides: number[]): ChartConfiguration["data"] => {
	let min = Math.min(...tides);
	let max = Math.max(...tides);
	let mid = (min + max) / 2;

	let labelColor = env.getCssVariable("--text-secondary");

	return {
		datasets: [
			{
				backgroundColor: "rgba(100, 149, 237, 0.15)",
				borderColor: "#6495ED",
				borderWidth: 2,
				data: tides,
				datalabels: { display: false },
				fill: true,
				pointRadius: 0,
				tension: 0.4,
			},
			{
				borderColor: "transparent",
				borderWidth: 0,
				data: tides.map(() => mid),
				datalabels: {
					align: "center",
					anchor: "center",
					color: labelColor,
					display: (context) => {
						let index = context.dataIndex;
						if (index === 0 || index === tides.length - 1) return false;

						let current = tides[index];
						let previous = tides[index - 1];
						let next = tides[index + 1];

						let isPeak = current > previous && current > next;
						let isTrough = current < previous && current < next;

						return isPeak || isTrough;
					},
					font: {
						family: "monospace",
						size: 10,
						weight: "bold",
					},
					formatter: (_value, context) => {
						let index = context.dataIndex;
						let height = tides[index];
						let hour = index;
						let period = hour >= 12 ? "pm" : "am";
						let hour12 = hour % 12 || 12;

						return `${height.toFixed(1)}m/${hour12}${period}`;
					},
				},
				offset: 10,
				pointRadius: 0,
			},
		],
		labels: tides.map((_, index) => index.toString()),
	};
};

export const getTideChartOptions = (tides: number[]): ChartConfiguration["options"] => {
	let min = Math.min(...tides);
	let max = Math.max(...tides);
	let delta = (max - min) * 0.05;

	return {
		layout: { padding: 0 },
		maintainAspectRatio: false,
		plugins: {
			legend: { display: false },
			tooltip: { enabled: false },
		},
		scales: {
			x: { display: false },
			y: {
				display: false,
				max: max + delta,
				min: min - delta,
			},
		},
	};
};
