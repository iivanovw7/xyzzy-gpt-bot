import type {
	DeleteLinkResponse,
	LinkCategoriesResponse,
	LinksListResponse,
	LinksQuery,
	LinkTagsResponse,
} from "@bindings";

import { logger } from "@/app/shared/logger";
import { HttpClient, HttpParams } from "@angular/common/http";
import { computed, inject, Injectable, signal } from "@angular/core";
import { catchError, finalize, of } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class LinksService {
	private http = inject(HttpClient);

	categories = signal<string[]>([]);
	deletingLinks = signal<Set<bigint | number>>(new Set());
	error = signal<boolean>(false);
	isLoading = signal<boolean>(false);
	linksResponse = signal<Nullable<LinksListResponse>>(null);

	status = computed(() => {
		switch (true) {
			case this.isLoading(): {
				return "loading";
			}
			case this.error(): {
				return "error";
			}
			case !!this.linksResponse(): {
				return "success";
			}
			default: {
				return "idle";
			}
		}
	});

	tags = signal<string[]>([]);

	deleteLink(id: bigint | number) {
		let currentDeleting = new Set(this.deletingLinks());
		currentDeleting.add(id);
		this.deletingLinks.set(currentDeleting);

		this.http
			.delete<QueryResponse<DeleteLinkResponse>>(`/links/${id}`)
			.pipe(
				finalize(() => {
					let newDeleting = new Set(this.deletingLinks());
					newDeleting.delete(id);
					this.deletingLinks.set(newDeleting);
				}),
				catchError((errorData) => {
					logger.error("LinksService delete error", errorData.message);

					return of(null);
				}),
			)
			.subscribe((response) => {
				if (response?.data.success) {
					let currentResponse = this.linksResponse();

					if (currentResponse) {
						this.linksResponse.set({
							...currentResponse,
							links: currentResponse.links.filter((l) => BigInt(l.id) !== BigInt(id)),
							totalCount: BigInt(currentResponse.totalCount) - BigInt(1),
						});
					}
				}
			});
	}

	fetchCategories() {
		this.http
			.get<QueryResponse<LinkCategoriesResponse>>("/links/categories")
			.pipe(
				catchError((errorData) => {
					logger.error("LinksService fetchCategories error", errorData.message);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.categories.set(response?.data.categories.map((c) => c.name) ?? []);
			});
	}

	fetchTags() {
		this.http
			.get<QueryResponse<LinkTagsResponse>>("/links/tags")
			.pipe(
				catchError((errorData) => {
					logger.error("LinksService fetchTags error", errorData.message);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.tags.set(response?.data.tags ?? []);
			});
	}

	queryLinks(filters: Partial<LinksQuery>) {
		this.isLoading.set(true);
		this.error.set(false);

		let parameters = new HttpParams();

		if (filters.search) {
			parameters = parameters.set("search", filters.search);
		}

		if (filters.category) {
			parameters = parameters.set("category", filters.category);
		}

		if (filters.tag) {
			parameters = parameters.set("tag", filters.tag);
		}

		if (filters.limit) {
			parameters = parameters.set("limit", filters.limit.toString());
		}

		if (filters.offset) {
			parameters = parameters.set("offset", filters.offset.toString());
		}

		this.http
			.get<QueryResponse<LinksListResponse>>("/links", {
				params: parameters,
			})
			.pipe(
				finalize(() => this.isLoading.set(false)),
				catchError((errorData) => {
					logger.error("LinksService error", errorData.message);
					this.error.set(true);

					return of(null);
				}),
			)
			.subscribe((response) => {
				this.linksResponse.set(response?.data ?? null);
			});
	}
}
