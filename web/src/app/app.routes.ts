import { inject } from '@angular/core';
import { CanActivateFn, Router, Routes } from '@angular/router';
import { UserService } from './core/auth/services/user.service';
import { map } from 'rxjs';

export const notAuth: CanActivateFn = () => {
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
    },
    {
        path: 'login',
        loadComponent: () => import('./core/auth/auth.component'),
        canActivate: [notAuth],
    },
];
