import { Component, DestroyRef, inject } from "@angular/core";

@Component({
	imports: [],
	selector: "app-auth-page",
	styleUrl: "./auth.component.scss",
	templateUrl: "./auth.component.html",
})
export default class AuthComponent {
	destroyRef = inject(DestroyRef);
}
