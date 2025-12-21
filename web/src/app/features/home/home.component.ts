import { Component, inject } from "@angular/core";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";

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
	private readonly authService = inject(AuthService);
	protected readonly user = this.authService.currentUser;
}
