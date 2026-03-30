import { Component, inject } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { TuiRoot } from "@taiga-ui/core";

import { LoadingService } from "./core/services/loading.service";

@Component({
	imports: [RouterOutlet, TuiRoot],
	selector: "app-root",
	standalone: true,
	templateUrl: "./app.component.html",
})
export class AppComponent {
	protected readonly loadingService = inject(LoadingService);
}
