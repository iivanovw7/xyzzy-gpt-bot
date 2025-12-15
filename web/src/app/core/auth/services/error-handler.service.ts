import { Injectable, inject, NgZone } from '@angular/core';
import { Router } from '@angular/router';
import { UserService } from './user.service';

@Injectable({ providedIn: 'root' })
export class ErrorHandlerService {
    private router = inject(Router);
    private zone = inject(NgZone);
    private userService = inject(UserService);

    handleUnauthorized(errorStatus: number): void {
        if (errorStatus === 401 || errorStatus === 403) {
            // this.userService.purgeUser();
        }
    }
}
