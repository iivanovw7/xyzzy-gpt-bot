import { Component, DestroyRef, inject, OnInit } from "@angular/core";

@Component({
	selector: "app-auth-page",
	templateUrl: "./auth.component.html",
	styleUrl: "./auth.component.scss",
	imports: [],
})
export default class AuthComponent implements OnInit {
	code = "401";
	message = "Unauthorized";
	destroyRef = inject(DestroyRef);

	constructor() {}

	ngOnInit(): void {}
}
