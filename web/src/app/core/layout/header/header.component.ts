import { Component, input } from "@angular/core";

@Component({
	host: {
		class: "header",
	},
	selector: "header[app-layout-header]",
	standalone: true,
	styleUrl: "./header.component.scss",
	templateUrl: "./header.component.html",
})
export default class HeaderComponent {
	title = input("web");
}
