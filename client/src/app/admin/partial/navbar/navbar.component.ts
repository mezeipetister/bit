import { Component, OnInit } from '@angular/core';
import { LoginService } from 'src/app/services/login/login.service';
import { Router, Event, NavigationEnd, ActivatedRoute, ParamMap } from '@angular/router';
import { Observable, Subscription } from 'rxjs';
import { switchMap, map } from 'rxjs/operators';
import { ErrorResponse } from 'src/app/class/error-response';
import { Notification } from 'src/app/class/notification';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.sass']
})
export class NavbarComponent implements OnInit {

  notifications: Notification[] = [];
  repository_id: String = null;

  constructor(
    private loginService: LoginService,
    private router: Router,
    private route: ActivatedRoute
  ) {
    // Register username observer
    this.loginService.userName.subscribe((val) => this.username = val);

    this.routerObserver = router.events.subscribe((e: Event) => {
      /**
       * IF Router Event
       */
      if (e instanceof NavigationEnd) {
        this.isActive = false;

        // TODO: REFACT PLEASE! Very ugly block
        let pathArray = this.route.snapshot['_routerState']['url'].split('/');
        this.repository_id = null;
        if (pathArray.length >= 3) {
          if (pathArray[1] == 'repository') {
            if (pathArray[2].length > 0) {
              this.repository_id = pathArray[2];
            }
          }
        }
        // this.route.firstChild.firstChild.paramMap.subscribe(
        //   (params: ParamMap): void => {
        //     this.repository_id = params.get("id");
        //   }
        // );
      }
    });
  }

  isActive: boolean = false;
  quick$: Observable<ErrorResponse>;
  routerObserver: Subscription;
  username: string;

  ngOnInit() {
    this.loginService.getUserName();
  }

  logout() {
    let url = this.router.url;
    this.loginService.logout(url);
    this.router.navigateByUrl('/login');
  }

}
