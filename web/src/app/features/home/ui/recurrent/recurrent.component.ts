import type { RecurrentPayment } from "@bindings";

import ButtonComponent from "@/app/shared/ui/components/button/button.component";
import IconComponent from "@/app/shared/ui/components/icon/icon.component";
import InputComponent from "@/app/shared/ui/components/input/input.component";
import ProgressBarComponent from "@/app/shared/ui/components/progress-bar/progress-bar.component";
import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";

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

	pay(item: RecurrentPayment, amountInput: number | string) {
		let amount = typeof amountInput === "string" ? parseFloat(amountInput) : amountInput;

		if (isNaN(amount) || amount <= 0) return;

		this.recurrentService.postTransaction(
			{
				amount,
				category: item.categoryId,
				description: item.description,
			},
			() => {
				// Transaction successful
			},
		);
	}
}
