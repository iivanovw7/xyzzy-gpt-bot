import { inject } from "@angular/core";
import { CanActivateFn, Router, Routes, UrlTree } from "@angular/router";
import { AuthService } from "./core/auth/services/auth.service";

type GuardResult = boolean | UrlTree;

export const authGuard: CanActivateFn = (): GuardResult => {
	const authService = inject(AuthService);
	const router = inject(Router);
	const isAuth = authService.isAuthenticated();
	console.log("authGuard", isAuth);
	return isAuth ? true : router.createUrlTree(["/login"]);
};

export const notAuthGuard: CanActivateFn = (): GuardResult => {
	const authService = inject(AuthService);
	const router = inject(Router);
	const isAuth = authService.isAuthenticated();
	console.log("notAuthGuard", isAuth);
	return isAuth ? router.createUrlTree(["/"]) : true;
};

export const routes: Routes = [
	{
		path: "",
		loadComponent: () => import("./features/profile/profile.component"),
		canActivate: [authGuard],
	},
	{
		path: "login",
		loadComponent: () => import("./core/auth/auth.component"),
		canActivate: [notAuthGuard],
	},
];
