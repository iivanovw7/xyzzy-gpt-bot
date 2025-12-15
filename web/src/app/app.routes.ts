import { inject } from '@angular/core';
import { CanActivateFn, Router, Routes, UrlTree } from '@angular/router';
import { UserService } from './core/auth/services/user.service';
import { map, Observable } from 'rxjs';

export const authGuard: CanActivateFn = (): Observable<boolean | UrlTree> => {
    const userService = inject(UserService);
    const router = inject(Router);

    return userService.isAuthenticated.pipe(
        map((isAuth) => (isAuth ? true : router.createUrlTree(['/login']))),
    );
};

export const notAuthGuard: CanActivateFn = () => {
    const userService = inject(UserService);
    const router = inject(Router);

    return userService.isAuthenticated.pipe(
        map((isAuth) => (isAuth ? router.createUrlTree(['/']) : true)),
    );
};

export const routes: Routes = [
    {
        path: '',
        loadComponent: () => import('./features/profile/profile.component'),
        canActivate: [authGuard],
    },
    {
        path: 'login',
        loadComponent: () => import('./core/auth/auth.component'),
        canActivate: [notAuthGuard],
    },
];
