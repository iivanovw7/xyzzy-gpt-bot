import { Chart, registerables } from "chart.js";
import type { ChartConfiguration } from "chart.js";
import ChartDataLabels from "chartjs-plugin-datalabels";

import type { ElementRef } from "@angular/core";

import { afterNextRender, Component, effect, input, signal, ViewChild } from "@angular/core";

Chart.register(...registerables, ChartDataLabels);

@Component({
	host: {
		class: "chart",
	},
	selector: "div[app-chart]",
	standalone: true,
	styleUrl: "./chart.components.scss",
	templateUrl: "./chart.component.html",
})
export default class ChartComponent {
	private chart?: Chart;

	private chartReady = signal(false);

	@ViewChild("chartCanvas") canvas!: ElementRef<HTMLCanvasElement>;
	config = input.required<ChartConfiguration["data"]>();
	options = input<ChartConfiguration["options"]>();

	type = input.required<"bar" | "doughnut" | "line">();

	constructor() {
		afterNextRender(() => {
			this.initChart();
			this.chartReady.set(true);
		});

		effect(() => {
			if (!this.chartReady()) return;

			let data = this.config();
			let options = this.options();

			if (!this.chart) return;

			this.chart.data = data;
			this.chart.options = {
				maintainAspectRatio: false,
				responsive: true,
				...options,
			};

			this.chart.update();
		});
	}

	private initChart() {
		let context = this.canvas.nativeElement.getContext("2d");

		if (!context) return;

		this.chart = new Chart(context, {
			data: this.config(),
			options: {
				maintainAspectRatio: false,
				plugins: {
					legend: {
						display: true,
						labels: { color: "#bdc3c7", font: { size: 12 } },
					},
				},
				responsive: true,
				...this.options(),
			},
			type: this.type(),
		});
	}
}
