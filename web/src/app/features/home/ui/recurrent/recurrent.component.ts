import type { RecurrentDashboard, RecurrentPayment } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import IconComponent from "@/app/shared/ui/components/icon/icon.component";
import { NotificationService } from "@/app/shared/ui/components/notification/notification.service";
import ProgressBarComponent from "@/app/shared/ui/components/progress-bar/progress-bar.component";
import { CommonModule } from "@angular/common";
import { Component, inject, input, signal } from "@angular/core";
import { FormsModule } from "@angular/forms";
import { TuiTextfield } from "@taiga-ui/core";

import { RecurrentService } from "../../service/recurrent.service";

@Component({
	host: {
		class: "recurrent-page",
	},
	imports: [CommonModule, FormsModule, TuiTextfield, ButtonComponent, IconComponent, ProgressBarComponent],
	selector: "div[app-recurrent]",
	styleUrl: "./recurrent.component.scss",
	templateUrl: "./recurrent.component.html",
})
export default class RecurrentComponent {
	protected loadingItems = signal<Set<bigint>>(new Set());
	protected readonly notificationService = inject(NotificationService);
	protected readonly recurrentService = inject(RecurrentService);

	readonly dashboard = input.required<Nullable<RecurrentDashboard>>();
	readonly title = input.required<string>();

	pay(item: RecurrentPayment, amountInput: number | string) {
		let amount = typeof amountInput === "string" ? parseFloat(amountInput) : amountInput;

		if (isNaN(amount) || amount <= 0) return;

		let finalAmount = item.isIncome ? amount : -amount;

		this.loadingItems.update((set) => {
			let newSet = new Set(set);

			newSet.add(item.id);

			return newSet;
		});

		this.recurrentService.postTransaction(
			{
				amount: finalAmount,
				category: item.categoryId,
				description: item.description,
			},
			() => {
				this.notificationService.success(`Payment of ${amount}€ recorded for ${item.description}`, {
					label: "Payment Successful",
				});
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
