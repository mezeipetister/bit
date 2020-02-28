import { Component, OnInit } from '@angular/core';
import { LoginService } from 'src/app/services/login/login.service';
import { Router } from '@angular/router';
import { catchError } from 'rxjs/operators';
import { HttpErrorResponse } from '@angular/common/http';
import { HttpError } from 'src/app/class/http-error';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.sass']
})
export class LoginComponent implements OnInit {

  username: string;
  password: string;

  error: HttpError | null = null;
  isLoading: boolean = false;

  constructor(private loginService: LoginService, private router: Router) { }

  ngOnInit() {

  }

  login() {
    this.isLoading = true;
    this.loginService.login(this.username, this.password).subscribe((r) => {
      this.isLoading = false;
      if (r) {
        let redirect = this.loginService.redirectUrl
          ? this.router.parseUrl(this.loginService.redirectUrl)
          : '/';
        this.router.navigateByUrl(redirect);
      }
    },
      (error) => {
        this.isLoading = false;
        this.error = error;
      })
  }

}
