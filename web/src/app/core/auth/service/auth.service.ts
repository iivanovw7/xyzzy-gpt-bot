import type { Signal, WritableSignal } from "@angular/core";
import type { LoginResponse } from "@bindings";
import type { Observable } from "rxjs";

import { config } from "@/app/shared/config";
import { env } from "@/app/shared/env";
import { logger } from "@/app/shared/logger";
import { routePath } from "@/app/shared/routes";
import { tokenStorage } from "@/app/shared/storage";
import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { Router } from "@angular/router";
import { BehaviorSubject, interval, of } from "rxjs";
import { catchError, filter, map, switchMap, take, tap } from "rxjs/operators";

import type { User } from "../model";

@Injectable({
	providedIn: "root",
})
export class AuthService {
	private accessTokenSignal: WritableSignal<Nullable<string>> = signal(tokenStorage.getAccessToken());
	private currentUserSignal: WritableSignal<Nullable<User>> = signal(null);

	private http = inject(HttpClient);
	private router = inject(Router);

	public currentUser: Signal<Nullable<User>> = this.currentUserSignal.asReadonly();
	public isAuthenticated: Signal<boolean> = computed(() => !!this.accessTokenSignal());
	public isRefreshing = false;
	public refreshToken$: BehaviorSubject<null | string> = new BehaviorSubject<null | string>(null);

	private saveAccessToken(token: string): void {
		tokenStorage.setAccessToken(token);
		this.accessTokenSignal.set(token);
	}

	private setUser(user: User): void {
		this.currentUserSignal.set(user);
	}

	private startTokenRefreshTimer(): void {
		if (this.isRefreshing) return;

		interval(config.net.tokenRefreshPeriod)
			.pipe(
				filter(() => this.isAuthenticated()),
				switchMap(() => {
					return this.refreshToken().pipe(
						catchError((error) => {
							logger.error("Token refresh failed, user logged out.", error.message);

							return of(null);
						}),
					);
				}),
			)
			.subscribe();
	}

	public getAccessToken(): null | string {
		return this.accessTokenSignal();
	}

	public login(): Observable<Nullable<LoginResponse>> {
		let initData = env.telegramInitData;

		if (!initData) {
			logger.error("No Telegram initData found. Are you running inside Telegram?");

			this.router.navigate([routePath.login]);

			return of(null);
		}

		return this.http
			.post<LoginResponse>(
				"/auth/login",
				{ initData },
				{
					withCredentials: true,
				},
			)
			.pipe(
				tap((response) => {
					this.saveAccessToken(response.accessToken);
					this.startTokenRefreshTimer();
					this.setUser({ id: response.userId });
				}),
				catchError((errorData) => {
					this.logout();
					this.router.navigate([routePath.login]);

					logger.error("Login error", errorData.message);

					return of(null);
				}),
			);
	}

	public logout(): void {
		tokenStorage.removeAccessToken();
		this.accessTokenSignal.set(null);
		this.currentUserSignal.set(null);
	}

	public refreshToken(): Observable<Nullable<LoginResponse>> {
		if (this.isRefreshing) {
			return this.refreshToken$.asObservable().pipe(
				filter((token): token is string => !!token),
				take(1),
				map(
					(token): LoginResponse => ({
						accessToken: token,
						userId: this.currentUser()?.id ?? "",
					}),
				),
			);
		}

		this.isRefreshing = true;
		this.refreshToken$.next(null);

		return this.http
			.post<LoginResponse>(
				"/auth/refresh",
				{},
				{
					withCredentials: true,
				},
			)
			.pipe(
				tap((response) => {
					this.setUser({ id: response.userId });
					this.saveAccessToken(response.accessToken);
					this.isRefreshing = false;
					this.refreshToken$.next(response.accessToken);
				}),
				catchError((errorData) => {
					logger.warn("Refresh token failed", errorData);

					return this.login().pipe(
						catchError((loginError) => {
							this.logout();
							this.router.navigate([routePath.login]);
							logger.error("Re-login failed", loginError);

							return of(null);
						}),
					);
				}),
			);
	}
}
