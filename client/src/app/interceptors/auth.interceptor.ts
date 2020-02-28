import { LoginService } from '../services/login/login.service';
import { Injectable } from '@angular/core';
import { HttpInterceptor, HttpRequest, HttpHandler } from '@angular/common/http';
import { Ok, Err, Result } from 'ts-results';
import { environment } from '../../environments/environment';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

    constructor(private loginService: LoginService) { }

    intercept(req: HttpRequest<any>, next: HttpHandler) {
        // Get the auth token from the service.
        const authToken = this.loginService.getToken();

        // Clone the request and replace the original headers with
        // cloned headers, updated with the authorization.
        const authReq = req.clone({
            // Set dev API url in DEV mode
            // TODO: remove condition
            url: (environment.production ? '/api' : '/api') + req.url,
            headers: req.headers.set('Token', authToken.ok ? authToken.unwrap() : '')
        });

        // send cloned request with header to the next handler.
        return next.handle(authReq);
    }
}