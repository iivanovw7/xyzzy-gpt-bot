import type { OnDestroy, OnInit } from "@angular/core";

import { Directive, ElementRef, HostListener, inject, Input } from "@angular/core";

import { logger } from "../logger";
import { ScrollSpyService } from "../services/scroll-spy.service";

@Directive({
	selector: "[appScrollTo], [appScrollSpyTarget]",
	standalone: true,
})
export class ScrollToDirective implements OnInit, OnDestroy {
	private element = inject(ElementRef);
	private scrollOffset: number = 50;
	private scrollSpyService = inject(ScrollSpyService);

	@Input() scrollContainer: string = "";
	@Input("appScrollSpyTarget") scrollSpyTargetId: string = "";
	@Input("appScrollTo") targetId: string = "";

	ngOnDestroy(): void {
		if (this.scrollSpyTargetId) {
			this.scrollSpyService.unregisterElement(this.scrollSpyTargetId);
		}
	}

	ngOnInit(): void {
		if (this.scrollSpyTargetId) {
			this.element.nativeElement.id = this.scrollSpyTargetId;
			this.scrollSpyService.registerElement(this.scrollSpyTargetId, this.element.nativeElement);
		}
	}

	@HostListener("click", ["$event"])
	onClick(event: Event): void {
		event.preventDefault();

		if (!this.targetId) return;

		let target = document.getElementById(this.targetId);

		if (!target) {
			logger.warn(`Element with id "${this.targetId}" not found.`);

			return;
		}

		if (this.scrollContainer) {
			let container = document.querySelector<HTMLElement>(this.scrollContainer);

			if (!container) {
				console.warn(`Scroll container "${this.scrollContainer}" not found.`);

				return;
			}

			let top =
				target.getBoundingClientRect().top -
				container.getBoundingClientRect().top +
				container.scrollTop -
				this.scrollOffset;

			container.scrollTo({
				behavior: "smooth",
				top,
			});

			return;
		}

		let top = window.scrollY + target.getBoundingClientRect().top - this.scrollOffset;

		window.scrollTo({
			behavior: "smooth",
			top,
		});
	}
}
