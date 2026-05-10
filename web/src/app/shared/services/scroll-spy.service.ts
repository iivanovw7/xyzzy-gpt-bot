import type { OnDestroy } from "@angular/core";
import type { Observable } from "rxjs";

import { inject, Injectable, NgZone } from "@angular/core";
import { BehaviorSubject } from "rxjs";

@Injectable({
	providedIn: "root",
})
export class ScrollSpyService implements OnDestroy {
	private activeSectionSubject = new BehaviorSubject<null | string>(null);

	private elementsToObserve = new Map<string, HTMLElement>();

	private ngZone = inject(NgZone);
	private observer: IntersectionObserver;

	public activeSection$: Observable<null | string> = this.activeSectionSubject.asObservable();

	constructor() {
		this.observer = this.createObserver();
	}

	private createObserver(): IntersectionObserver {
		return new IntersectionObserver(
			(entries) => {
				this.ngZone.run(() => {
					for (let entry of entries) {
						let element = entry.target as HTMLElement;
						let id = element.dataset["scrollSpyId"];

						if (!id) continue;

						if (entry.isIntersecting) {
							this.activeSectionSubject.next(id);
						} else if (this.activeSectionSubject.getValue() === id) {
							this.activeSectionSubject.next(null);
						}
					}
				});
			},
			{
				root: null,
				rootMargin: "-50px 0px -90% 0px",
				threshold: 0,
			},
		);
	}

	ngOnDestroy(): void {
		this.observer.disconnect();
		this.activeSectionSubject.complete();
	}

	registerElement(id: string, element: HTMLElement): void {
		if (this.elementsToObserve.has(id)) return;

		element.dataset["scrollSpyId"] = id;

		this.elementsToObserve.set(id, element);

		this.ngZone.runOutsideAngular(() => {
			this.observer.observe(element);
		});
	}

	unregisterElement(id: string): void {
		let element = this.elementsToObserve.get(id);

		if (!element) return;

		this.observer.unobserve(element);

		delete element.dataset["scrollSpyId"];

		this.elementsToObserve.delete(id);

		if (this.activeSectionSubject.getValue() === id) {
			this.activeSectionSubject.next(null);
		}
	}
}
