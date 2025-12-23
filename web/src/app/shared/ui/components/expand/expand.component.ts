import type { OnInit } from "@angular/core";

import { NgTemplateOutlet } from "@angular/common";
import { ChangeDetectionStrategy, Component, contentChild, input, signal, TemplateRef } from "@angular/core";
import { TuiItem } from "@taiga-ui/cdk/directives/item";

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	host: {
		"(transitionend.self)": "onTransitionEnd($any($event))",
		"[class.expand--expanded]": "expanded()",
		"[class.expand--open]": "open()",
		class: "expand",
	},
	imports: [NgTemplateOutlet],
	selector: "div[app-expand]",
	standalone: true,
	styleUrl: "./expand.component.scss",
	templateUrl: "./expand.component.html",
})
export default class ExpandComponent implements OnInit {
	protected readonly content = contentChild(TuiItem, { read: TemplateRef });
	protected readonly open = signal(false);

	public readonly expanded = input(false);

	protected onTransitionEnd(event: TransitionEvent): void {
		if (event.propertyName === "grid-template-rows") {
			this.open.set(this.expanded());
		}
	}

	public ngOnInit(): void {
		this.open.set(this.expanded());
	}
}
