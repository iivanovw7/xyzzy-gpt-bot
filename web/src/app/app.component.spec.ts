import { TestBed } from "@angular/core/testing";

import { AppComponent } from "./app.component";

describe("AppComponent", () => {
	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [AppComponent],
		}).compileComponents();
	});

	it("should create the app", () => {
		let fixture = TestBed.createComponent(AppComponent);
		let app = fixture.componentInstance;
		expect(app).toBeTruthy();
	});
});
