import { tap, map, catchError } from 'rxjs/operators';
import { Injectable } from '@angular/core';
import { HttpInterceptor, HttpRequest, HttpHandler, HttpErrorResponse, HttpResponse } from '@angular/common/http';
import { ErrorResponse } from '../class/error-response';
import { throwError } from 'rxjs';
import { HttpError } from '../class/http-error';
// import { Router } from '@angular/router';

@Injectable()
export class ErrorHandlerInterceptor implements HttpInterceptor {
    constructor() { }

    private handleError(error: HttpErrorResponse) {
        if (error.error instanceof ErrorEvent) {
            // A client-side or network error occurred. Handle it accordingly.
            return throwError(new HttpError('danger', 'Oooo'))
        } else {
            if (error.status == 0) {
                return throwError(
                    new HttpError('danger', 'A Gardenova szerver nem elérhető! \
                    Vagy a szerver hibás, vagynincs internet kapcsolat'))
            }
            // If there is 401 redirect to login
            if (error.status == 401) {
                // this.router.navigateByUrl('/login');
            }
            if (error.status == 400) {
                if (error.error instanceof ErrorResponse) {
                    // If standard API error
                    return throwError(new HttpError('warning', error.error.message))
                } else {
                    // If unknown API error format
                    return throwError(new HttpError('warning', error.error.message))
                }
            } else {
                if (error.error instanceof ErrorResponse) {
                    // If standard API error
                    return throwError(new HttpError('danger', error.error.message))
                } else {
                    // If unknown API error format
                    return throwError(new HttpError('danger', error.error.message))
                }
            }
        }
    };

    intercept(req: HttpRequest<any>, next: HttpHandler) {
        return next.handle(req)
            .pipe(
                catchError(this.handleError)
            );
    }
}