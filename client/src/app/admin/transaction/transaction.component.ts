import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Transaction } from 'src/app/class/transaction';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-transaction',
  templateUrl: './transaction.component.html',
  styleUrls: ['./transaction.component.sass']
})
export class TransactionComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: Transaction[] = [];

  constructor(private http: HttpClient, private params: RouterParamService) { }

  ngOnInit() {
    this.http.get<Transaction[]>("/repository/" + this.repository_id + "/transaction/all")
      .subscribe(val => this.model = val);
  }

}
