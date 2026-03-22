import { cond, divide, min, multiply, pipe, subtract, T } from "ramda";

export const formatSurfWeekday = (timestamp: string) => {
	let date = new Date(timestamp);
	let intl = new Intl.DateTimeFormat("en-US", {
		weekday: "short",
	});

	return intl.format(date);
};

export const formatSurfDate = (timestamp: string) => {
	let date = new Date(timestamp);
	let intl = new Intl.DateTimeFormat("en-US", {
		day: "numeric",
		month: "short",
	});

	return intl.format(date);
};

export const formatTime = (timestamp: string) => {
	let date = new Date(timestamp);
	let intl = new Intl.DateTimeFormat("en-US", {
		hour: "numeric",
		hour12: true,
		minute: "2-digit",
	});

	return intl.format(date);
};
const calculateHue = (values: [number, number, number, number]) => {
	let [base, offset, range, drop] = values;

	return pipe(
		(value: number) => subtract(value, offset),
		(diff: number) => divide(diff, range),
		(ratio: number) => multiply(ratio, drop),
		(reduction: number) => subtract(base, reduction),
	);
};

const calculateCappedHue = (values: [number, number, number, number, number]) => {
	let [base, offset, range, drop, cap] = values;

	return pipe(
		(value: number) => subtract(value, offset),
		(diff: number) => divide(diff, range),
		(ratio: number) => min(ratio, cap),
		(capped: number) => multiply(capped, drop),
		(reduction: number) => subtract(base, reduction),
	);
};

const getHue = cond([
	[(value: number) => value < 200, calculateHue([220, 0, 200, 90])],
	[(value: number) => value < 400, calculateHue([130, 200, 200, 70])],
	[(value: number) => value < 600, calculateHue([60, 400, 200, 30])],
	[(value: number) => value < 1000, calculateHue([30, 600, 400, 30])],
	[T, calculateCappedHue([360, 1000, 1000, 60, 1])],
]);

export const getEnergyColor = (energy: number) => {
	let hue = getHue(energy);

	return `hsl(${hue}, 80%, 50%)`;
};
