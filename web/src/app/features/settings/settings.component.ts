import type { Theme } from "@/types/styles.types";

import { ThemeService } from "@/app/core/services/theme.service";
import { CommonModule } from "@angular/common";
import { Component, effect, inject } from "@angular/core";
import { FormControl, ReactiveFormsModule } from "@angular/forms";
import { TuiDataList, TuiDropdown, TuiTextfield } from "@taiga-ui/core";
import { TuiDataListWrapper, TuiSelect } from "@taiga-ui/kit";

import HeaderComponent from "../../core/layout/header/header.component";
import NavigationComponent from "../../core/navigation/navigation.component";

@Component({
	host: {
		class: "settings-page",
	},
	imports: [
		HeaderComponent,
		NavigationComponent,
		CommonModule,
		ReactiveFormsModule,
		TuiSelect,
		TuiDataList,
		TuiDataListWrapper,
		TuiTextfield,
		TuiDropdown,
	],
	selector: "section[app-settins-page]",
	styleUrl: "./settings.component.scss",
	templateUrl: "./settings.component.html",
})
export default class SettingsComponent {
	private readonly themeService = inject(ThemeService);

	public readonly themeControl = new FormControl<Theme>(this.themeService.theme(), {
		nonNullable: true,
	});
	public readonly themes: Theme[] = ["light", "dark"];

	constructor() {
		effect(() => {
			let currentTheme = this.themeService.theme();

			if (this.themeControl.value !== currentTheme) {
				this.themeControl.setValue(currentTheme, { emitEvent: false });
			}
		});

		this.themeControl.valueChanges.subscribe((theme) => {
			this.themeService.setTheme(theme);
		});
	}
}
