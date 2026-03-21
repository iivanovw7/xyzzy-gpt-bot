import { mergeRight, pipe, prop } from "ramda";

import type { SysInfoResponse } from "@bindings";

import { BYTES_IN_GB, SECONDS_IN_DAY, SECONDS_IN_HOUR, SECONDS_IN_MINUTE } from "@/app/shared/time";
import { HttpClient } from "@angular/common/http";
import { Component, inject } from "@angular/core";
import { toSignal } from "@angular/core/rxjs-interop";
import { map } from "rxjs/operators";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";

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
		class: "home-page",
	},
	imports: [HeaderComponent, NavigationComponent],
	selector: "section[app-home-page]",
	styleUrl: "./home.components.scss",
	templateUrl: "./home.component.html",
})
export default class HomeComponent {
	private readonly http = inject(HttpClient);
	protected readonly sysInfo = toSignal(
		this.http.get<{ data: SysInfoResponse }>("/sysinfo").pipe(map(pipe(prop("data"), enhanceSysInfo))),
	);

	private readonly authService = inject(AuthService);

	protected readonly user = this.authService.currentUser;
}
