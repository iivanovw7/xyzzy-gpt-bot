import { HttpClient } from "@angular/common/http";
import { computed, inject, Injectable, Signal, signal, WritableSignal } from "@angular/core";
import { Router } from "@angular/router";
import { BehaviorSubject, EMPTY, interval, Observable, of, throwError } from "rxjs";
import { catchError, filter, switchMap, tap } from "rxjs/operators";
import { TokenStorage } from "../../storage/token.storage";
import { User } from "./auth.model";

interface TokenResponse {
	access_token: string;
	user_id: string;
}

@Injectable({
	providedIn: "root",
})
export class AuthService {
	private tokenStorage = inject(TokenStorage);
	private router = inject(Router);

	public isRefreshing = false;

	public refreshToken$: BehaviorSubject<string | null> = new BehaviorSubject<string | null>(null);

	private accessTokenSignal: WritableSignal<string | null> = signal(this.getInitialAccessToken());

	public isAuthenticated: Signal<boolean> = computed(() => !!this.accessTokenSignal());

	private currentUserSignal: WritableSignal<Nullable<User>> = signal(null);

	public currentUser: Signal<Nullable<User>> = this.currentUserSignal.asReadonly();

	private getInitialAccessToken(): string | null {
		return this.tokenStorage.getAccessToken();
	}

	private extractTokenFromUrl(): string | null {
		const urlParams = new URLSearchParams(window.location.search);

		let token = urlParams.get("token");

		if (token) {
			console.log("JWT token successfully extracted.");
		} else {
			console.warn("No JWT token found in URL. Development/Manual access assumed.");
		}

		return token;
	}

	private cleanTokenFromUrl(): void {
		const url = new URL(window.location.href);

		if (url.searchParams.has("token")) {
			url.searchParams.delete("token");
			window.history.replaceState({}, document.title, url.toString());
			console.log("Initial token successfully removed from URL.");
		}
	}

	constructor(private http: HttpClient) {}

	public getAccessToken(): string | null {
		return this.accessTokenSignal();
	}

	private setUser(user: User): void {
		this.currentUserSignal.set(user);
	}

	private saveAccessToken(token: string): void {
		this.tokenStorage.setAccessToken(token);
		this.accessTokenSignal.set(token);
	}

	public logout(): void {
		this.tokenStorage.removeAccessToken();
		this.accessTokenSignal.set(null);
		this.currentUserSignal.set(null);
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
					this.setUser({ user_id: response.user_id });
				}),
				catchError(() => {
					this.logout();
					this.cleanTokenFromUrl();
					this.router.navigate(["login"]);

					return EMPTY;
				}),
			);
	}

	private startTokenRefreshTimer(): void {
		const refreshIntervalMs = 60 * 1000;

		if (this.isRefreshing) return;

		interval(refreshIntervalMs)
			.pipe(
				filter(() => this.isAuthenticated()),
				switchMap(() => {
					console.log(`[Token Timer] Proactively attempting token refresh...`);

					return this.refreshToken().pipe(
						catchError((err) => {
							console.error("[Token Timer] Refresh failed, user logged out.", err);

							return of(null);
						}),
					);
				}),
			)
			.subscribe();
	}

	public refreshToken(): Observable<any> {
		if (this.isRefreshing) {
			return this.refreshToken$
				.asObservable()
				.pipe(switchMap((token) => (token ? of(token) : throwError(() => "Refresh failed"))));
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
					this.saveAccessToken(response.access_token);
					this.setUser({ user_id: response.user_id });
					this.isRefreshing = false;
					this.refreshToken$.next(response.access_token);
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
