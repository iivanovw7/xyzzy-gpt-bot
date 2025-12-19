export const env = {
	getCssVariable: (variable: string) => {
		// prettier-ignore
		return getComputedStyle(document.documentElement)
            .getPropertyValue(variable)
            .trim();
	},
	/**
	 * Refers to a current document `html` element.
	 */
	html: document.documentElement,
	/**
	 * True if runs in browser environment.
	 */
	isBrowser: Boolean(typeof window !== "undefined" && window.document.createElement),
	/**
	 * Refers true if dark theme is enabled,
	 */
	isDarkTheme: window.matchMedia("(prefers-color-scheme: dark)").matches,
	/**
	 * Equals `true` is running in development mode.
	 */
	isMediaQuerySupported: Boolean("matchMedia" in window && typeof window.matchMedia === "function"),
	/**
	 * Root portal container.
	 */
	portal: document.getElementById("portal")! as HTMLDivElement,
};
