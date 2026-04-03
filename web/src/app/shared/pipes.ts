import { isNil } from "ramda";

import type { PipeTransform } from "@angular/core";

import { Pipe } from "@angular/core";

@Pipe({
	name: "compactNumber",
	standalone: true,
})
export class CompactNumberPipe implements PipeTransform {
	transform(value: number): string {
		if (isNil(value)) return "";

		let intl = new Intl.NumberFormat("en", {
			maximumFractionDigits: 0,
			notation: "compact",
		});

		return intl.format(value);
	}
}
