import type { Signal, WritableSignal } from "@angular/core";
import type { Observable } from "rxjs";

import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { Router } from "@angular/router";
import { BehaviorSubject, EMPTY, interval, of, throwError } from "rxjs";
import { catchError, filter, map, switchMap, take, tap } from "rxjs/operators";

import type { User } from "./auth.model";

import { LoggerService } from "../../../shared/services/log/log.service";
import { TokenStorage } from "../../storage/token.storage";

type TokenResponse = {
	access_token: string;
	user_id: string;
};

@Injectable({
	providedIn: "root",
})
export class AuthService {
	private accessTokenSignal: WritableSignal<null | string> = signal(this.getInitialAccessToken());
	private currentUserSignal: WritableSignal<Nullable<User>> = signal(null);
	private http = inject(HttpClient);

	private log = inject(LoggerService);
	private router = inject(Router);

	private tokenStorage = inject(TokenStorage);

	public currentUser: Signal<Nullable<User>> = this.currentUserSignal.asReadonly();

	public isAuthenticated: Signal<boolean> = computed(() => !!this.accessTokenSignal());

	public isRefreshing = false;

	public refreshToken$: BehaviorSubject<null | string> = new BehaviorSubject<null | string>(null);

	private cleanTokenFromUrl(): void {
		let url = new URL(window.location.href);

		if (url.searchParams.has("token")) {
			url.searchParams.delete("token");
			window.history.replaceState({}, document.title, url.toString());
		}
	}

	private extractTokenFromUrl(): null | string {
		let urlParameters = new URLSearchParams(window.location.search);

		let token = urlParameters.get("token");

		if (token) {
			this.log.info("JWT token successfully extracted.");
		} else {
			this.log.warn("No JWT token found in URL. Development/Manual access assumed.");
		}

		return token;
	}

	private getInitialAccessToken(): null | string {
		return this.tokenStorage.getAccessToken();
	}

	private saveAccessToken(token: string): void {
		this.tokenStorage.setAccessToken(token);
		this.accessTokenSignal.set(token);
	}

	private setUser(user: User): void {
		this.currentUserSignal.set(user);
	}

	private startTokenRefreshTimer(): void {
		let refreshIntervalMs = 60 * 1000;

		if (this.isRefreshing) return;

		interval(refreshIntervalMs)
			.pipe(
				filter(() => this.isAuthenticated()),
				switchMap(() => {
					return this.refreshToken().pipe(
						catchError((error) => {
							this.log.error("Token refresh failed, user logged out.", error.message);

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

	public login(): Observable<TokenResponse> {
		let initialUrlToken = this.extractTokenFromUrl();

		if (!initialUrlToken) {
			this.router.navigate(["login"]);

			return EMPTY;
		}

		return this.http
			.get<TokenResponse>("/auth/login", {
				headers: { Authorization: `Bearer ${initialUrlToken}` },
				withCredentials: true,
			})
			.pipe(
				tap((response) => {
					this.saveAccessToken(response.access_token);
					this.startTokenRefreshTimer();
					this.cleanTokenFromUrl();
					this.setUser({ id: response.user_id });
				}),
				catchError(() => {
					this.logout();
					this.cleanTokenFromUrl();
					this.router.navigate(["login"]);

					return EMPTY;
				}),
			);
	}

	public logout(): void {
		this.tokenStorage.removeAccessToken();
		this.accessTokenSignal.set(null);
		this.currentUserSignal.set(null);
	}

	public refreshToken(): Observable<string> {
		if (this.isRefreshing) {
			return this.refreshToken$.asObservable().pipe(
				filter((token): token is string => !!token),
				take(1),
			);
		}

		this.isRefreshing = true;
		this.refreshToken$.next(null);

		return this.http
			.post<TokenResponse>(
				"/auth/refresh",
				{},
				{
					withCredentials: true,
				},
			)
			.pipe(
				tap((response) => {
					this.setUser({ id: response.user_id });
				}),
				map((response) => response.access_token),
				tap((token) => {
					this.saveAccessToken(token);
					this.isRefreshing = false;
					this.refreshToken$.next(token);
				}),
				catchError((error) => {
					this.logout();
					this.isRefreshing = false;
					this.refreshToken$.next(null);

					return throwError(() => error);
				}),
			);
	}
}
