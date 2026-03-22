import { mergeRight } from "ramda";

import type { SysInfoResponse } from "@bindings";

import { BYTES_IN_GB, SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE } from "@/app/shared/time";
import { Component, computed, inject } from "@angular/core";

import { SysInfoService } from "../../service/sysinfo.service";

const formatCpu = (cpu: number) => `${cpu.toFixed(1)}%`;

const formatMemory = (bytes: bigint | number | string) => {
	return `${(Number(bytes) / BYTES_IN_GB).toFixed(2)} GB`;
};

const formatUptime = (secondsString: bigint | number | string) => {
	let seconds = Number(secondsString);
	let days = Math.floor(seconds / SECONDS_IN_DAY);
	let hours = Math.floor((seconds % SECONDS_IN_DAY) / SECONDS_IN_HOUR);
	let mins = Math.floor((seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE);

	return `${days}d ${hours}h ${mins}m`;
};

const formatDatabaseLatency = (ms: number) => {
	return `${ms.toFixed(2)} ms`;
};

const enhanceSysInfo = (info: SysInfoResponse) => {
	return mergeRight(info, {
		formattedCpu: formatCpu(info.cpuUsage),
		formattedDbLatency: formatDatabaseLatency(info.dbLatencyMs),
		formattedTotalMem: formatMemory(info.totalMem),
		formattedUptime: formatUptime(info.uptime),
		formattedUsedMem: formatMemory(info.usedMem),
	});
};

@Component({
	host: {
		class: "sysinfo-page",
	},
	selector: "div[app-sysinfo]",
	styleUrl: "./sysinfo.component.scss",
	templateUrl: "./sysinfo.component.html",
})
export default class SysInfoComponent {
	private readonly sysInfoService = inject(SysInfoService);

	protected readonly sysInfo = computed(() => {
		let info = this.sysInfoService.sysInfo();

		return info ? enhanceSysInfo(info) : null;
	});
}
