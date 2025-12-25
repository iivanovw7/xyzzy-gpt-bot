import { env } from "@/app/shared/env";

const TOTAL_CHART_COLORS = 20;

export const getChartPalette = () => {
	return Array.from({ length: TOTAL_CHART_COLORS }, (_, index) => env.getCssVariable(`--chart${index + 1}`));
};
