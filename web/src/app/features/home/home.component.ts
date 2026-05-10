import type { OnInit } from "@angular/core";

import { ScrollToDirective } from "@/app/shared/directives/scroll-to.directive";
import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import IconComponent from "@/app/shared/ui/components/icon/icon.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";
import { ScrollSpyService } from "../../shared/services/scroll-spy.service";
import { RecurrentService } from "./service/recurrent.service";
import { SurfService } from "./service/surf.service";
import { SysInfoService } from "./service/sysinfo.service";
import RecurrentComponent from "./ui/recurrent/recurrent.component";
import SurfComponent from "./ui/surf/surf.component";
import SysInfoComponent from "./ui/sysinfo/sysinfo.component";

@Component({
	host: {
		class: "home-page",
		id: "home-page",
	},
	imports: [
		HeaderComponent,
		NavigationComponent,
		SurfComponent,
		SysInfoComponent,
		RecurrentComponent,
		SkeletonComponent,
		ButtonComponent,
		CommonModule,
		IconComponent,
		ScrollToDirective,
	],
	selector: "section[app-home-page]",
	styleUrl: "./home.components.scss",
	templateUrl: "./home.component.html",
})
export default class HomeComponent implements OnInit {
	protected readonly scrollSpyService = inject(ScrollSpyService);
	protected readonly activeSectionId$ = this.scrollSpyService.activeSection$;
	protected readonly recurrentService = inject(RecurrentService);
	protected readonly surfService = inject(SurfService);
	protected readonly sysInfoService = inject(SysInfoService);
	private readonly authService = inject(AuthService);

	protected readonly user = this.authService.currentUser;

	ngOnInit() {
		this.sysInfoService.querySysInfo();
		this.surfService.querySurfReport();
		this.recurrentService.queryRecurrent();
	}
}
