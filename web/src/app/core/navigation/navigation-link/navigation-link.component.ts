import { CommonModule } from "@angular/common";
import { Component, computed, inject, input } from "@angular/core";
import { Router } from "@angular/router";

import type { Size } from "../../../../types/styles.types";
import type { IconKey } from "../../../shared/ui";

import { IconComponent } from "../../../shared/ui";

@Component({
	imports: [IconComponent, CommonModule],
	selector: "app-navigation-link",
	styleUrl: "./navigation-link.component.scss",
	templateUrl: "./navigation-link.component.html",
})
export default class NavigationLinkComponent {
	readonly dataActive = computed(() => (this.isActive ? "" : null));
	disabled = input(false);
	href = input.required<string>();
	icon = input<IconKey>();
	replace = input(false);
	router = inject(Router);
	size = input<Size>("medium");
	readonly tabIndex = computed<Nullable<number>>(() => (this.isActive ? -1 : null));
	text = input.required<string>();

	get isActive(): boolean {
		return this.router.isActive(this.href(), {
			fragment: "ignored",
			matrixParams: "ignored",
			paths: "exact",
			queryParams: "ignored",
		});
	}
}
