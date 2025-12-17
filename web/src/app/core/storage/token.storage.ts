import { inject, Injectable } from "@angular/core";

import { StorageService } from "./storage.service";
import { StorageKey } from "./storage.types";

@Injectable({ providedIn: "root" })
export class TokenStorage {
	private storage = inject(StorageService);

	getAccessToken(): Nullable<string> {
		return this.storage.get(StorageKey.AccessToken);
	}

	removeAccessToken() {
		this.storage.delete(StorageKey.AccessToken);
	}

	setAccessToken(token: Nullable<string>) {
		this.storage.set(StorageKey.AccessToken, token);
	}
}
