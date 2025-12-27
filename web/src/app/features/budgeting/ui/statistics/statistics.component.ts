import type { OnInit } from "@angular/core";

import { CommonModule } from "@angular/common";
import { Component, inject } from "@angular/core";

import { StatisticsService } from "./service/statistics.service";
import TransactionsComponent from "./ui/transactions.component";

@Component({
	host: {
		class: "budgeting__statistics",
	},
	imports: [CommonModule, TransactionsComponent],
	selector: "div[app-budgeting-statistics]",
	styleUrl: "./statistics.component.scss",
	templateUrl: "./statistics.component.html",
})
export default class BudgetingStatisticsComponent implements OnInit {
	protected readonly service = inject(StatisticsService);

	ngOnInit() {
		this.service.queryTransactions({ category: null, description: null });
	}
}
