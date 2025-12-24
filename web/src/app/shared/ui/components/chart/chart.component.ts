import { Chart, registerables } from "chart.js";
import type { ChartConfiguration } from "chart.js";
import ChartDataLabels from "chartjs-plugin-datalabels";

import type { ElementRef } from "@angular/core";

import { afterNextRender, Component, input, ViewChild } from "@angular/core";

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

	@ViewChild("chartCanvas") canvas!: ElementRef<HTMLCanvasElement>;
	config = input.required<ChartConfiguration["data"]>();
	options = input<ChartConfiguration["options"]>();
	type = input.required<"bar" | "doughnut" | "line">();

	constructor() {
		afterNextRender(() => {
			let data = this.config();

			if (this.chart) {
				this.chart.data = data;
				this.chart.update();
			} else {
				this.initChart();
			}
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
