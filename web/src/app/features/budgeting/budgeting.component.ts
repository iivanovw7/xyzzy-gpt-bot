import { Component, inject, signal } from "@angular/core";

import { AuthService } from "../../core/auth";
import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";
import TabsComponent from "../../shared/ui/components/tabs/tabs.component";
import BudgetingOverveiwComponent from "./overview/overview.component";
import BudgetingStatisticsComponent from "./statistics/statistics.component";

@Component({
	host: {
		class: "budgeting-page",
	},
	imports: [
		HeaderComponent,
		NavigationComponent,
		TabsComponent,
		BudgetingOverveiwComponent,
		BudgetingStatisticsComponent,
	],
	selector: "section[app-budgeting-page]",
	styleUrl: "./budgeting.component.scss",
	templateUrl: "./budgeting.component.html",
})
export default class BudgetingComponent {
	private readonly authService = inject(AuthService);
	protected readonly user = this.authService.currentUser;

	activeTab = signal(0);
}
