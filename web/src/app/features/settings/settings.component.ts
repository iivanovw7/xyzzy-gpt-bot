import type { OnInit } from "@angular/core";

import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";

import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";

@Component({
	host: {
		class: "settings-page",
	},
	imports: [HeaderComponent, NavigationComponent, CommonModule],
	selector: "section[app-settins-page]",
	styleUrl: "./settings.component.scss",
	templateUrl: "./settings.component.html",
})
export default class SettingsComponent implements OnInit {
	// eslint-disable-next-line @angular-eslint/no-empty-lifecycle-method
	ngOnInit() {}
}
