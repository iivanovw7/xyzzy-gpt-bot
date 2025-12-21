import { Component, DestroyRef, inject } from "@angular/core";

@Component({
	host: {
		class: "login-page",
	},
	imports: [],
	selector: "section[app-login-page]",
	styleUrl: "./login.component.scss",
	templateUrl: "./login.component.html",
})
export default class LoginComponent {
	destroyRef = inject(DestroyRef);
}
