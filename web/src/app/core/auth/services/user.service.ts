import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject, distinctUntilChanged, map, Observable, shareReplay, tap } from 'rxjs';
import { User, UserResponse } from '../user.model';
import { JwtService } from './jwt.service';

@Injectable({ providedIn: 'root' })
export class UserService {
    private currentUserSubject = new BehaviorSubject<User | null>(null);

    public currentUser = this.currentUserSubject.asObservable().pipe(distinctUntilChanged());

    public isAuthenticated = this.currentUser.pipe(map((user) => !!user));

    constructor(
        private http: HttpClient,
        private jwtService: JwtService,
    ) {}

    getCurrentUser(): Observable<{ data: User }> {
        return this.http.get<UserResponse>('/user').pipe(
            tap({
                next: ({ data }) => this.setUser(data),
                error: () => this.purgeUser(),
            }),
            shareReplay(1),
        );
    }

    setUser(user: User): void {
        this.currentUserSubject.next(user);
    }

    purgeUser(): void {
        this.jwtService.destroyToken();
        this.currentUserSubject.next(null);
    }
}
