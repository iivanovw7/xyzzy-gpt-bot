import { HttpInterceptorFn } from '@angular/common/http';
import { environment } from '../../../../environments/environment';

export const apiInterceptor: HttpInterceptorFn = (req, next) => {
    let apiUrl = environment.API_BASE_URL;

    const apiReq = req.clone({
        url: `${apiUrl}/api${req.url}`,
    });

    return next(apiReq);
};
