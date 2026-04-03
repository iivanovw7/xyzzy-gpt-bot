import type { TuiPortalContext } from "@taiga-ui/cdk/portals";
import type { TuiNotificationOptions } from "@taiga-ui/core";

import { ChangeDetectionStrategy, Component } from "@angular/core";
import { injectContext } from "@taiga-ui/polymorpheus";

import type { NotificationType } from "./notification.types";

type NotificationData = {
	label: string;
	message: string;
	type: NotificationType;
};

@Component({
	changeDetection: ChangeDetectionStrategy.OnPush,
	selector: "app-notification",
	standalone: true,
	templateUrl: "./notification.component.html",
})
export class NotificationComponent {
	public readonly context = injectContext<TuiPortalContext<TuiNotificationOptions<NotificationData>, void>>();

	public get label(): string {
		return this.context.data.label || "";
	}

	public get message(): string {
		return this.context.data.message || "";
	}
}
