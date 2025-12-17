import { Injectable, inject } from "@angular/core";
import { StorageService } from "./storage.service";
import { StorageKey } from "./storage.types";

@Injectable({ providedIn: "root" })
export class TokenStorage {
	private storage = inject(StorageService);

	setAccessToken(token: Nullable<string>) {
		this.storage.set(StorageKey.AccessToken, token);
	}

	getAccessToken(): Nullable<string> {
		return this.storage.get(StorageKey.AccessToken);
	}

	removeAccessToken() {
		this.storage.delete(StorageKey.AccessToken);
	}
}
