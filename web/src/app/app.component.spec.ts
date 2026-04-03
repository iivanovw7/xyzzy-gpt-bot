import { TestBed } from "@angular/core/testing";
import { provideAnimations } from "@angular/platform-browser/animations";
import { provideRouter } from "@angular/router";
import { provideTaiga } from "@taiga-ui/core";

import { AppComponent } from "./app.component";

describe("AppComponent", () => {
	beforeAll(() => {
		Object.defineProperty(window, "matchMedia", {
			configurable: true,
			value: vi.fn().mockImplementation((query: string) => ({
				addEventListener: vi.fn(),
				addListener: vi.fn(),
				dispatchEvent: vi.fn(),
				matches: false,
				media: query,
				onchange: null,
				removeEventListener: vi.fn(),
				removeListener: vi.fn(),
			})),
			writable: true,
		});
	});

	beforeEach(async () => {
		await TestBed.configureTestingModule({
			imports: [AppComponent],
			providers: [provideAnimations(), provideTaiga(), provideRouter([])],
		}).compileComponents();
	});

	it("should create the app", () => {
		let fixture = TestBed.createComponent(AppComponent);
		let app = fixture.componentInstance;
		expect(app).toBeTruthy();
	});
});
