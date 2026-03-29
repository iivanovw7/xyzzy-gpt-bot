import { ChangeDetectionStrategy, Component, input, model } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { TuiIcon, TuiTextfield } from "@taiga-ui/core";

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	imports: [FormsModule, TuiTextfield, TuiIcon],
	selector: "app-input",
	standalone: true,
	styleUrl: "./input.component.scss",
	templateUrl: "./input.component.html",
})
export default class InputComponent {
	readonly disabled = input(false);
	readonly isSearch = input(false);
	readonly placeholder = input<string>("");
	readonly size = input<"m" | "s">("m");
	readonly type = input<string>("text");
	readonly value = model<number | string>("");
}
