import { inject, Injectable } from "@angular/core";
import { TuiNotificationService } from "@taiga-ui/core";
import { PolymorpheusComponent } from "@taiga-ui/polymorpheus";
import { take } from "rxjs";

import type { NotificationOptions, NotificationType } from "./notification.types";

import { NotificationComponent } from "./notification.component";

@Injectable({
	providedIn: "root",
})
export class NotificationService {
	private readonly notifications = inject<TuiNotificationService>(TuiNotificationService);

	private getAppearance(type: NotificationType): string {
		switch (type) {
			case "error":
				return "negative";
			case "info":
				return "info";
			case "simple":
				return "neutral";
			case "success":
				return "positive";
			default:
				return "info";
		}
	}

	public error(message: string, options: Omit<NotificationOptions, "type"> = {}): void {
		this.show(message, { ...options, type: "error" });
	}

	public info(message: string, options: Omit<NotificationOptions, "type"> = {}): void {
		this.show(message, { ...options, type: "info" });
	}

	public show(message: string, options: NotificationOptions = {}): void {
		let { label = "", type = "info" } = options;

		this.notifications
			.open(new PolymorpheusComponent(NotificationComponent), {
				appearance: this.getAppearance(type),
				autoClose: options.autoClose ?? 3000,
				data: { label, message, type },
			})
			.pipe(take(1))
			.subscribe();
	}

	public simple(message: string, options: Omit<NotificationOptions, "type"> = {}): void {
		this.show(message, { ...options, type: "simple" });
	}

	public success(message: string, options: Omit<NotificationOptions, "type"> = {}): void {
		this.show(message, { ...options, type: "success" });
	}
}
