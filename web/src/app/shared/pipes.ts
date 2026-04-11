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

@Pipe({
	name: "highlight",
	standalone: true,
})
export class HighlightPipe implements PipeTransform {
	transform(value: null | string | undefined, search: null | string): string {
		if (!value) return "";

		let safeValue = value.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");

		if (!search) return safeValue;

		let safeSearch = search.replaceAll(/[\s#$()*+,.?[\\\]^{|}-]/g, "\\$&");
		let regex = new RegExp(`(${safeSearch})`, "gi");

		return safeValue.replace(regex, `<mark class="highlight">$1</mark>`);
	}
}
