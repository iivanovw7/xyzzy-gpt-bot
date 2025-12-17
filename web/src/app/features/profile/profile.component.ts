import { Component, inject } from "@angular/core";

import { AuthService } from "../../core/auth/services/auth.service";

@Component({
	imports: [],
	selector: "app-profile-page",
	styleUrl: "./profile.components.css",
	templateUrl: "./profile.component.html",
})
export default class ProfileComponent {
	private readonly authService = inject(AuthService);
	protected readonly user = this.authService.currentUser;
}
