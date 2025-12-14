import { Component, DestroyRef, inject, OnInit } from '@angular/core';

@Component({
    selector: 'app-auth-page',
    templateUrl: './auth.component.html',
    imports: [],
})
export default class AuthComponent implements OnInit {
    title = '';
    destroyRef = inject(DestroyRef);

    constructor() {}

    ngOnInit(): void {
        this.title = 'Login';
    }
}
