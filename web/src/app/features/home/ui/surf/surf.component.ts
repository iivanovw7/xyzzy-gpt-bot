import type { SurfReportResponse } from "@bindings";

import { DecimalPipe } from "@angular/common";
import { Component, computed, inject } from "@angular/core";

import ChartComponent from "../../../../shared/ui/components/chart/chart.component";
import IconComponent from "../../../../shared/ui/components/icon/icon.component";
import { SurfService } from "../../service/surf.service";
import { formatSurfDate, formatSurfWeekday, formatTime, getEnergyColor } from "./lib/data-formatters";
import { getTideChartConfig, getTideChartOptions } from "./lib/tidal-chart";

const enhanceSurfReport = (report: SurfReportResponse) => {
	return {
		...report,
		daily: report.daily.map((day) => {
			let date = new Date(day.timestamp);
			let dayOfWeek = date.getDay();
			let isWeekend = dayOfWeek === 0 || dayOfWeek === 6;

			return {
				...day,
				energyColor: getEnergyColor(day.swellEnergy),
				formattedDate: formatSurfDate(day.timestamp),
				formattedSunrise: formatTime(day.sunrise),
				formattedSunset: formatTime(day.sunset),
				formattedWeekday: formatSurfWeekday(day.timestamp),
				isWeekend,
				swellArrowRotation: (day.swellDirection + 180) % 360,
				tideChartConfig: getTideChartConfig(day.hourlyTides),
				tideChartOptions: getTideChartOptions(day.hourlyTides),
				windArrowRotation: (day.windDirection + 180) % 360,
			};
		}),
	};
};

@Component({
	host: {
		class: "surf-forecast",
	},
	imports: [DecimalPipe, IconComponent, ChartComponent],
	selector: "div[app-surf-forecast]",
	styleUrl: "./surf.component.scss",
	templateUrl: "./surf.component.html",
})
export default class SurfComponent {
	private readonly surfService = inject(SurfService);

	protected readonly surfReport = computed(() => {
		let report = this.surfService.surfReport();

		return report ? enhanceSurfReport(report) : null;
	});
}
