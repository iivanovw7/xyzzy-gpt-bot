import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";

import type { MenuItem } from "../../shared/routes";

import { menuItems } from "../../shared/routes";
import NavigationLinkComponent from "./navigation-link/navigation-link.component";

@Component({
	host: {
		class: "navigation",
	},
	imports: [NavigationLinkComponent, CommonModule],
	selector: "nav[app-navigation]",
	styleUrl: "./navigation.component.scss",
	templateUrl: "./navigation.component.html",
})
export default class NavigationComponent {
	menuItems: MenuItem[] = menuItems;
}
