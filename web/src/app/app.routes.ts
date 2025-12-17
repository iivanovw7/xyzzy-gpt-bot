import type { Routes, UrlTree } from "@angular/router";

import { inject } from "@angular/core";
import { Router } from "@angular/router";

import { AuthService } from "./core/auth/services/auth.service";

type GuardResult = boolean | UrlTree;
type AuthGuardCallback = (isAuth: boolean, router: Router) => GuardResult;

const authGuard = (callback: AuthGuardCallback) => {
	return () => callback(inject(AuthService).isAuthenticated(), inject(Router));
};

export const routes: Routes = [
	{
		canActivate: [
			authGuard((isAuth, router) => {
				return isAuth ? true : router.createUrlTree(["/login"]);
			}),
		],
		loadComponent: () => import("./features/profile/profile.component"),
		path: "",
	},
	{
		canActivate: [
			authGuard((isAuth, router) => {
				return isAuth ? router.createUrlTree(["/"]) : true;
			}),
		],
		loadComponent: () => import("./core/auth/auth.component"),
		path: "login",
	},
];
