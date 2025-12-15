import { HttpErrorResponse, HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { ErrorHandlerService } from '../auth/services/error-handler.service';

export const errorInterceptor: HttpInterceptorFn = (req, next) => {
    const errorHandler = inject(ErrorHandlerService);

    return next(req).pipe(
        catchError((error: HttpErrorResponse) => {
            if (error.status === 401 || error.status === 403) {
                errorHandler.handleUnauthorized(error.status);

                return throwError(() => error);
            }

            return throwError(() => error);
        }),
    );
};
