import type { OnInit } from "@angular/core";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";
import { SurfService } from "./service/surf.service";
import { SysInfoService } from "./service/sysinfo.service";
import SurfComponent from "./ui/surf/surf.component";
import SysInfoComponent from "./ui/sysinfo/sysinfo.component";

@Component({
	host: {
		class: "home-page",
	},
	imports: [
		HeaderComponent,
		NavigationComponent,
		SurfComponent,
		SysInfoComponent,
		SkeletonComponent,
		ButtonComponent,
		CommonModule,
	],
	selector: "section[app-home-page]",
	styleUrl: "./home.components.scss",
	templateUrl: "./home.component.html",
})
export default class HomeComponent implements OnInit {
	protected readonly surfService = inject(SurfService);
	protected readonly sysInfoService = inject(SysInfoService);
	private readonly authService = inject(AuthService);
	protected readonly user = this.authService.currentUser;

	ngOnInit() {
		this.sysInfoService.querySysInfo();
		this.surfService.querySurfReport();
	}
}
