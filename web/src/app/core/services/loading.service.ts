import { Injectable, signal } from "@angular/core";

@Injectable({
	providedIn: "root",
})
export class LoadingService {
	private readonly _activeRequests = signal(0);

	readonly isLoading = signal(false);

	setLoading(isLoading: boolean): void {
		if (isLoading) {
			this._activeRequests.update((count) => count + 1);
		} else {
			this._activeRequests.update((count) => Math.max(0, count - 1));
		}

		this.isLoading.set(this._activeRequests() > 0);
	}
}
