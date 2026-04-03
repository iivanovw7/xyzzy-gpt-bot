import { ChangeDetectionStrategy, Component, contentChild, input, model, signal, TemplateRef } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { TuiItem } from "@taiga-ui/cdk/directives/item";
import { TuiDataList, TuiDropdown, TuiTextfield } from "@taiga-ui/core";
import { TuiFilterByInputPipe } from "@taiga-ui/core";
import { TuiComboBox, TuiDataListWrapper } from "@taiga-ui/kit";

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	host: {
		class: "combobox",
	},
	imports: [
		FormsModule,
		TuiComboBox,
		TuiDataListWrapper,
		TuiFilterByInputPipe,
		TuiTextfield,
		TuiDataList,
		TuiDropdown,
	],
	selector: "app-combobox",
	standalone: true,
	styleUrl: "./combobox.component.scss",
	templateUrl: "./combobox.component.html",
})
export default class ComboboxComponent {
	protected readonly content = contentChild(TuiItem, { read: TemplateRef });
	protected readonly open = signal(false);

	readonly iconStart = input<string>("");
	readonly isClearable = input<boolean>(true);
	readonly items = input<string[]>([]);
	readonly placeholder = input<string>("Select...");
	readonly size = input<"m" | "s">("m");

	readonly value = model<null | string>(null);
}
