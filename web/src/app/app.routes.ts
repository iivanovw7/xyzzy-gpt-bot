import type { Routes, UrlTree } from "@angular/router";

import { inject } from "@angular/core";
import { Router } from "@angular/router";

import { AuthService } from "./core/auth/service";
import { basePath, routePath } from "./shared/routes";

type GuardResult = boolean | UrlTree;
type AuthGuardCallback = (isAuth: boolean, router: Router) => GuardResult;

const authGuard = (callback: AuthGuardCallback) => {
	return () => callback(inject(AuthService).isAuthenticated(), inject(Router));
};

export const routes: Routes = [
	{
		canActivate: [
			authGuard((isAuth, router) => {
				return isAuth ? true : router.createUrlTree([routePath.login]);
			}),
		],
		loadComponent: () => import("./features/home/home.component"),
		path: basePath.home,
	},
	{
		canActivate: [
			authGuard((isAuth, router) => {
				return isAuth ? router.createUrlTree([routePath.home]) : true;
			}),
		],
		loadComponent: () => import("./features/login/login.component"),
		path: basePath.login,
	},
	{
		canActivate: [
			authGuard((isAuth, router) => {
				return isAuth ? true : router.createUrlTree([routePath.login]);
			}),
		],
		loadComponent: () => import("./features/budgeting/budgeting.component"),
		path: basePath.budgeting,
	},
];
