import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { Account, AccountNew } from 'src/app/class/account';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
    selector: 'app-account-new',
    templateUrl: './account-new.component.html',
    styleUrls: ['./account-new.component.css']
})
export class AccountNewComponent implements OnInit {

    id: string = this.params.hasParam("repository_id");
    model: AccountNew = new AccountNew();

    constructor(
        private http: HttpClient,
        private route: ActivatedRoute,
        private router: Router,
        private params: RouterParamService) { }

        submit() {
            this.model.id = this.model.id.trim();
            this.http.put<Account>("/repository/" + this.id + "/account/new", this.model)
            .subscribe(val => this.router.navigateByUrl("/repository/" + this.id + "/account"));
        }

        ngOnInit() {
        }

}
