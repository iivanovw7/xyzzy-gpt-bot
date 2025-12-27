import type { TuiStringMatcher } from "@taiga-ui/cdk/types";

import { ChangeDetectionStrategy, Component, contentChild, input, model, signal, TemplateRef } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { TuiItem } from "@taiga-ui/cdk/directives/item";
import { TuiDataList, TuiTextfield } from "@taiga-ui/core";
import { TuiComboBox, TuiDataListWrapper, TuiFilterByInputPipe } from "@taiga-ui/kit";

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	host: {
		class: "combobox",
	},
	imports: [FormsModule, TuiComboBox, TuiDataListWrapper, TuiFilterByInputPipe, TuiTextfield, TuiDataList],
	selector: "app-combobox",
	standalone: true,
	styleUrl: "./combobox.component.scss",
	templateUrl: "./combobox.component.html",
})
export default class ComboboxComponent {
	protected readonly content = contentChild(TuiItem, { read: TemplateRef });
	protected readonly matcher: TuiStringMatcher<string> = (item, query) => {
		return item.toLowerCase().includes(query.toLowerCase());
	};
	protected readonly open = signal(false);

	readonly items = input<string[]>([]);

	readonly placeholder = input<string>("Select...");

	readonly value = model<null | string>(null);
}
