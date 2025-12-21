import { Component, inject } from "@angular/core";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";

@Component({
	host: {
		class: "budgeting-page",
	},
	imports: [HeaderComponent, NavigationComponent],
	selector: "section[app-budgeting-page]",
	styleUrl: "./budgeting.component.scss",
	templateUrl: "./budgeting.component.html",
})
export default class BudgetingComponent {
	private readonly authService = inject(AuthService);
	protected readonly user = this.authService.currentUser;
}
