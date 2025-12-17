import { Component, inject, OnInit } from "@angular/core";
import { AuthService } from "../../core/auth/services/auth.service";

@Component({
	selector: "app-profile-page",
	templateUrl: "./profile.component.html",
	imports: [],
	styleUrl: "./profile.components.css",
})
export default class ProfileComponent implements OnInit {
	private readonly authService = inject(AuthService);

	protected readonly user = this.authService.currentUser;

	ngOnInit(): void {}
}
