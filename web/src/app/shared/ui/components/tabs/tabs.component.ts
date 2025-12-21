import { CommonModule } from "@angular/common";
import { Component, input, model } from "@angular/core";
import { TuiTabs } from "@taiga-ui/kit";

@Component({
	host: {
		class: "tabs",
		role: "tablist",
	},
	imports: [CommonModule, TuiTabs],
	selector: "nav[app-tabs], div[app-tabs]",
	standalone: true,
	styleUrl: "./tabs.component.scss",
	templateUrl: "./tabs.component.html",
})
export default class TabsComponent {
	activeItemIndex = model<number>(0);
	items = input.required<string[]>();
}
