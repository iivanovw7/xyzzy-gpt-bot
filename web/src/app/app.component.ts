import { Component } from "@angular/core";
import { RouterOutlet } from "@angular/router";

@Component({
	standalone: true,
	selector: "app-root",
	imports: [RouterOutlet],
	template: `<base href="/" /><router-outlet></router-outlet>
		<div id="portal"></div>`,
})
export class AppComponent {}
