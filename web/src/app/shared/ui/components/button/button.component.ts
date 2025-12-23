import type { Align, Color, Fill, Size } from "@/types/styles.types";

import { CommonModule } from "@angular/common";
import { Component, computed, input } from "@angular/core";

import type { IconKey } from "../icon";

import IconComponent from "../icon/icon.component";

@Component({
	host: {
		"[attr.aria-busy]": "isLoading()",
		"[attr.disabled]": 'disabled() ? "" : null',
		"[class.button--color-primary]": "color() === 'primary'",
		"[class.button--color-secondary]": "color() === 'secondary'",
		"[class.button--color-tertiary]": "color() === 'tertiary'",
		"[class.button--fill-full]": "fill() === 'full'",
		"[class.button--fill-none]": "fill() === 'none'",
		"[class.button--fill-outline]": "fill() === 'outline'",
		"[class.button--full-width]": "fullWidth()",
		"[class.button--icon-only]": "icon() && !text()",
		class: "button",
	},
	imports: [CommonModule, IconComponent],
	selector: "button[app-button], a[app-button]",
	standalone: true,
	styleUrl: "./button.component.scss",
	templateUrl: "./button.component.html",
})
export default class ButtonComponent {
	iconClass = input<string>("");
	iconPosition = input<Align>("center");
	protected iconFullClass = computed(() => {
		return `button__icon button__icon--align-${this.iconPosition()} ${this.iconClass()}`;
	});
	textAlign = input<Align>("start");
	textClass = input<string>("");

	protected textFullClass = computed(() => {
		return `button__text button__text--align-${this.textAlign()} ${this.textClass()}`;
	});
	color = input<Color>("primary");
	disabled = input<boolean>(false);

	fill = input<Fill>("full");

	fullWidth = input<boolean>(false);
	icon = input<Nullable<IconKey>>(null);

	iconSize = input<Size>("small");

	isLoading = input<boolean>(false);
	loaderClass = input<string>("");
	text = input<Nullable<string>>(null);
}
