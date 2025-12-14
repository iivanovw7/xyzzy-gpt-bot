import { Injectable } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class JwtService {
    getToken(): string {
        return this.extractTokenFromUrl() || window.localStorage['jwtToken'];
    }

    saveToken(token: string): void {
        window.localStorage['jwtToken'] = token;
    }

    destroyToken(): void {
        window.localStorage.removeItem('jwtToken');
    }

    extractTokenFromUrl(): string | null {
        const urlParams = new URLSearchParams(window.location.search);

        let token = urlParams.get('token');

        if (token) {
            this.saveToken(token);
            console.log('JWT token successfully extracted.');
        } else {
            console.warn('No JWT token found in URL. Development/Manual access assumed.');
        }

        return token;
    }
}
