import { CommonModule } from "@angular/common";
import { Component } from "@angular/core";

@Component({
	host: {
		class: "skeleton",
	},
	imports: [CommonModule],
	selector: "div[app-skeleton]",
	standalone: true,
	styleUrl: "./skeleton.component.scss",
	templateUrl: "./skeleton.component.html",
})
export default class SkeletonComponent {}
