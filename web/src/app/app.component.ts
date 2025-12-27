import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { TuiRoot } from "@taiga-ui/core";

@Component({
	imports: [RouterOutlet, TuiRoot, TuiRoot],
	selector: "app-root",
	standalone: true,
	templateUrl: "./app.component.html",
})
export class AppComponent {}
