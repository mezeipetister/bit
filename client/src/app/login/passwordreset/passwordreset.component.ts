import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ResetPassword } from 'src/app/class/reset-password';
import { ErrorResponse } from 'src/app/class/error-response';
import { Observable } from 'rxjs';
import { tap } from 'rxjs/operators';
import { HttpError } from 'src/app/class/http-error';

@Component({
  selector: 'app-passwordreset',
  templateUrl: './passwordreset.component.html',
  styleUrls: ['./passwordreset.component.css']
})
export class PasswordresetComponent implements OnInit {

  email: string = '';
  isValid: boolean = true;
  error: HttpError | null = null;
  isOk: boolean = false;
  isLoading: boolean = false;

  constructor(private http: HttpClient) { }

  ngOnInit() {
  }

  resetPassword() {
    if (this.email.length > 0 && this.email.includes('@')) {
      this.isValid = true;
      this.isLoading = true;
      this.http.post('/login/reset_password', new ResetPassword(this.email)).subscribe(
        () => this.isOk = true,
        (err) => {
          this.error = err;
          this.isLoading = false;
        })
    } else {
      this.isValid = false;
    }
  }

}
