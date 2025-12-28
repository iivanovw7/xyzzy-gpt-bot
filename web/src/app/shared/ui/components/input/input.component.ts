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
	readonly isSearch = input(false);
	readonly placeholder = input<string>("");
	readonly value = model<string>("");
}
