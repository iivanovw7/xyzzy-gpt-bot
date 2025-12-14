import { Component, DestroyRef, inject, OnInit, signal } from '@angular/core';
import { UserService } from '../../core/auth/services/user.service';
import { User } from '../../core/auth/user.model';
import { takeUntilDestroyed } from '@angular/core/rxjs-interop';

@Component({
    selector: 'app-profile-page',
    templateUrl: './profile.component.html',
    imports: [],
    styleUrl: './profile.components.css',
})
export default class ProfileComponent implements OnInit {
    protected readonly title = signal('web');
    protected readonly user = signal<User | null>(null);

    destroyRef = inject(DestroyRef);

    constructor(private readonly userService: UserService) {}

    ngOnInit(): void {
        this.userService.currentUser
            .pipe(takeUntilDestroyed(this.destroyRef))
            .subscribe((user) => this.user.set(user));
    }
}
