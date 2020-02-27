import { Injectable } from '@angular/core';
import { Router } from '@angular/router';

@Injectable({
  providedIn: 'root'
})
export class RouterParamService {

  constructor(private router: Router) { }

  routerParams() {
    var params = {};
    var route = this.router.routerState.snapshot.root;
    do {
      for (const [key, value] of Object.entries(route.params)) {
        params[key] = value;
      }
      // params.push(route.params);
      route = route.firstChild;
    } while (route);
    return params;
  }

  hasParam(key: string): string | null {
    let params = this.routerParams();
    return params[key] ? params[key] : null;
  }
}
