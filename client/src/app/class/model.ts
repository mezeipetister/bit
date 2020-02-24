import { HttpError } from './http-error';
import { Observable, interval, Subject } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { take } from 'rxjs/operators';

interface hasLength {
    length: number;
}

export class Model<T> {
    /**
     * Http client for Api call
     */
    private httpClient: HttpClient
    /**
     * Http service observable
     */
    private apiUrl: string;
    /**
     * Underlying form data
     */
    public data: T;
    /**
     * Submit status indicator
     */
    public isLoading: boolean = false;
    public isOk: boolean = false;
    public status: Subject<T> = new Subject();
    /**
     * Storing error if there any
     */
    public error: HttpError | null = null;
    public submit() {
        this.isLoading = true;
        this.httpClient.post<T>(this.apiUrl, this.data).subscribe({
            next: (val: T & hasLength) => {
                this.isLoading = false;
                this.isOk = true;
                // Animate the button if there is any
                interval(1500).pipe(take(1)).subscribe(() => this.isOk = false);
                // Notify subscibers about status change
                // Send the new data to them
                this.status.next(this.data);
                // If there is content
                // set it to the new data value
                if (val.length > 0) {
                    this.data = val;
                }
            },
            error: (err) => {
                this.isLoading = false;
                this.error = err;
                // this.status.error(Error("Submit failed"));
            }
        });
    }
    /**
     * Constructor
     * @param data: T
     */
    constructor(http: HttpClient, url: string, data: T) {
        this.httpClient = http;
        this.apiUrl = url;
        this.data = data;
    }
}
