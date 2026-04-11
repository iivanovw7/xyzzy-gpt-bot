import { Component, inject } from "@angular/core";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";
import LinksListComponent from "./ui/list/list.component";

@Component({
	host: {
		class: "links-page",
	},
	imports: [HeaderComponent, NavigationComponent, LinksListComponent],
	selector: "section[app-links-page]",
	styleUrl: "./links.component.scss",
	templateUrl: "./links.component.html",
})
export default class LinksComponent {
	private readonly authService = inject(AuthService);
	protected readonly user = this.authService.currentUser;
}
