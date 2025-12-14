import { HttpErrorResponse, HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { Router } from '@angular/router';
import { throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { UserService } from '../auth/services/user.service';

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
    const router = inject(Router);
    const userService = inject(UserService);

    return next(req).pipe(
        catchError((error: HttpErrorResponse) => {
            alert(error.message);
            if (error.status === 401 || error.status === 403) {
                console.error(
                    'Request failed with status:',
                    error.status,
                    '. Redirecting to login.',
                );

                userService.purgeUser();

                void router.navigate(['/login']);

                return throwError(() => 'Redirected due to unauthorized access');
            }

            return throwError(() => error);
        }),
    );
};
