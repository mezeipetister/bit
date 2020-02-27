import { Component, OnInit } from '@angular/core';
import { Account } from 'src/app/class/account';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-account-detail',
  templateUrl: './account-detail.component.html',
  styleUrls: ['./account-detail.component.css']
})
export class AccountDetailComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  account_id: string = this.params.hasParam("account_id");
  model: Account = new Account();

  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private params: RouterParamService) { }

  submit() {
    this.http.post<Account>("/repository/" + this.repository_id + "/account/" + this.account_id, this.model)
      .subscribe(val => {
        this.model = val;
        alert("Sikeresen mentve!");
      });
  }

  ngOnInit() {
    this.http.get<Account>("/repository/" + this.repository_id + "/account/" + this.account_id)
      .subscribe(val => this.model = val);
  }

}
