import { Component, OnInit } from '@angular/core';
import { Account } from 'src/app/class/account';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-account',
  templateUrl: './account.component.html',
  styleUrls: ['./account.component.css']
})
export class AccountComponent implements OnInit {

  id: string = this.params.hasParam("repository_id");
  model: Account[] = null;

  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private params: RouterParamService) { }

  ngOnInit() {
    this.http.get<Account[]>("/repository/" + this.id + "/account/all")
      .subscribe(val => this.model = val);
  }

}
