export type NotificationType = "error" | "info" | "simple" | "success";

export type NotificationOptions = {
	autoClose?: number;
	label?: string;
	type?: NotificationType;
};

export type InternalNotificationData = {
	label: string;
	message: string;
	type: NotificationType;
};
