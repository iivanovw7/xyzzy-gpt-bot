import type { RecurrentPayment } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import IconComponent from "@/app/shared/ui/components/icon/icon.component";
import InputComponent from "@/app/shared/ui/components/input/input.component";
import ProgressBarComponent from "@/app/shared/ui/components/progress-bar/progress-bar.component";
import { CommonModule } from "@angular/common";
import { Component, inject, signal } from "@angular/core";

import { RecurrentService } from "../../service/recurrent.service";

@Component({
	host: {
		class: "recurrent-page",
	},
	imports: [CommonModule, ButtonComponent, InputComponent, IconComponent, ProgressBarComponent],
	selector: "div[app-recurrent]",
	styleUrl: "./recurrent.component.scss",
	templateUrl: "./recurrent.component.html",
})
export default class RecurrentComponent {
	protected readonly recurrentService = inject(RecurrentService);
	protected dashboard = this.recurrentService.dashboard;
	protected loadingItems = signal<Set<bigint>>(new Set());

	pay(item: RecurrentPayment, amountInput: number | string) {
		let amount = typeof amountInput === "string" ? parseFloat(amountInput) : amountInput;

		if (isNaN(amount) || amount <= 0) return;

		this.loadingItems.update((set) => {
			let newSet = new Set(set);

			newSet.add(item.id);

			return newSet;
		});

		this.recurrentService.postTransaction(
			{
				amount,
				category: item.categoryId,
				description: item.description,
			},
			() => {
				// TODO: notify success
			},
			() => {
				this.loadingItems.update((set) => {
					let newSet = new Set(set);

					newSet.delete(item.id);

					return newSet;
				});
			},
		);
	}
}
