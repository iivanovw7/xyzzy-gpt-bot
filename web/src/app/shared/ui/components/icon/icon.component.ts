import type { Size } from "src/types/styles.types";

import { Component, computed, input } from "@angular/core";
import { LucideAngularModule } from "lucide-angular";

import type { IconKey } from "./icon.registry";

import { Icon } from "./icon.registry";

@Component({
	host: {
		"[class.icon--large]": 'size() === "large"',
		"[class.icon--medium]": 'size() === "medium"',
		"[class.icon--x-xmall]": 'size() === "x-small"',
		"[class.icon--xmall]": 'size() === "small"',
		class: "icon class()",
	},
	imports: [LucideAngularModule],
	selector: "app-icon",
	standalone: true,
	styleUrl: "./icon.component.scss",
	templateUrl: "./icon.component.html",
})
export default class IconComponent {
	protected icon = computed(() => Icon[this.name()]);

	class = input<string>("");
	name = input.required<IconKey>();
	size = input<Size>("medium");

	strokeWidth = input(2);
}
