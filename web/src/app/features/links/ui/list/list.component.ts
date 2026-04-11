import { config } from "@/app/shared/config";
import { HighlightPipe } from "@/app/shared/pipes";
import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import SkeletonComponent from "@/app/shared/ui/components/skeleton/skeleton.component";
import { CommonModule } from "@angular/common";
import { Component, effect, inject, signal, untracked } from "@angular/core";
import { toObservable, toSignal } from "@angular/core/rxjs-interop";
import { FormsModule } from "@angular/forms";
import { TuiButtonX, TuiDropdown, TuiFilterByInputPipe, TuiInput, TuiLink, TuiTextfield } from "@taiga-ui/core";
import { TuiComboBox, TuiDataListWrapper } from "@taiga-ui/kit";
import { debounceTime, distinctUntilChanged } from "rxjs";

import { LinksService } from "../../service/links.service";

@Component({
	host: {
		class: "links-list",
	},
	imports: [
		CommonModule,
		FormsModule,
		ButtonComponent,
		SkeletonComponent,
		TuiComboBox,
		TuiDataListWrapper,
		TuiFilterByInputPipe,
		TuiDropdown,
		TuiTextfield,
		TuiButtonX,
		TuiInput,
		TuiLink,
		HighlightPipe,
	],
	selector: "div[app-links-list]",
	styleUrl: "./list.component.scss",
	templateUrl: "./list.component.html",
})
export default class LinksListComponent {
	protected readonly searchTerm = signal<string>("");
	protected readonly selectedCategory = signal<Nullable<string>>(null);
	protected readonly selectedTag = signal<Nullable<string>>(null);

	protected readonly service = inject(LinksService);

	private readonly debouncedSearchTerm = toSignal(
		toObservable(this.searchTerm).pipe(debounceTime(config.ui.debounce), distinctUntilChanged()),
		{ initialValue: "" },
	);

	constructor() {
		this.service.fetchCategories();
		this.service.fetchTags();

		effect(() => {
			let search = this.debouncedSearchTerm() || null;
			let category = this.selectedCategory();
			let tag = this.selectedTag();

			untracked(() => {
				this.service.queryLinks({ category, search, tag });
			});
		});
	}

	protected handleReload() {
		this.service.queryLinks({
			category: this.selectedCategory(),
			search: this.searchTerm() || null,
			tag: this.selectedTag(),
		});
	}

	protected handleSearchClear(): void {
		this.searchTerm.set("");
	}
}
