import { ChangeDetectionStrategy, Component, input } from "@angular/core";
import { TuiProgressBar } from "@taiga-ui/kit";

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	imports: [TuiProgressBar],
	selector: "app-progress-bar",
	standalone: true,
	styleUrl: "./progress-bar.component.scss",
	templateUrl: "./progress-bar.component.html",
})
export default class ProgressBarComponent {
	readonly max = input<number>(100);
	readonly value = input<number>(0);
}
