import type { Size } from "src/types/styles.types";

import { ChangeDetectionStrategy, Component, computed, input } from "@angular/core";
import { TuiIcon } from "@taiga-ui/core";

import { Icon } from "./icon.registry";

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	host: {
		"[class.icon--large]": 'size() === "large"',
		"[class.icon--medium]": 'size() === "medium"',
		"[class.icon--small]": 'size() === "small"',
		"[class.icon--x-small]": 'size() === "x-small"',
		class: "icon",
	},
	imports: [TuiIcon],
	selector: "app-icon",
	standalone: true,
	styleUrl: "./icon.component.scss",
	templateUrl: "./icon.component.html",
})
export default class IconComponent {
	protected readonly iconName = computed(() => Icon[this.name()]);
	readonly name = input.required<keyof typeof Icon>();

	readonly size = input<Size>("medium");
}
