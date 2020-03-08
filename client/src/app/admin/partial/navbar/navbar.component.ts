import { Component, OnInit, HostListener } from '@angular/core';
import { LoginService } from 'src/app/services/login/login.service';
import { Router, Event, NavigationEnd, ActivatedRoute, ParamMap } from '@angular/router';
import { Observable, Subscription } from 'rxjs';
import { switchMap, map } from 'rxjs/operators';
import { ErrorResponse } from 'src/app/class/error-response';
import { Notification } from 'src/app/class/notification';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.sass']
})
export class NavbarComponent implements OnInit {

  notifications: Notification[] = [];
  repository_id: String = null;

  shortcutNewTransaction() {
    if (this.repository_id) {
      this.router.navigateByUrl("/repository/" + this.repository_id + "/transaction/new");
    }
  }

  constructor(
    private loginService: LoginService,
    private router: Router,
    private route: ActivatedRoute,
    private params: RouterParamService
  ) {
    // Register username observer
    this.loginService.userName.subscribe((val) => this.username = val);

    this.routerObserver = router.events.subscribe((e: Event) => {
      /**
       * IF Router Event
       */
      if (e instanceof NavigationEnd) {
        this.isActive = false;
        /**
         * Set repository id if exists
         */
        let params = this.params.routerParams();
        this.repository_id = params["repository_id"] ? params["repository_id"] : null;
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
