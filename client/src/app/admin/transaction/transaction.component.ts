import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Transaction } from 'src/app/class/transaction';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { getLocaleFirstDayOfWeek } from '@angular/common';

@Component({
  selector: 'app-transaction',
  templateUrl: './transaction.component.html',
  styleUrls: ['./transaction.component.sass']
})
export class TransactionComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: Transaction[] = [];
  account: string = "";
  from: string = this.getTodayWindow(-30);
  till: string = this.getTodayWindow();

  constructor(private http: HttpClient, private params: RouterParamService) { }

  getTodayWindow(window: number = 0): string {
    let dt: Date = new Date();
    dt.setDate(dt.getDate() + window);
    return dt.toISOString().split('T')[0];
  }

  loadData() {
    if (!this.from) {
      this.from = new Date().toISOString().split('T')[0];
    }
    if (!this.till) {
      this.till = new Date().toISOString().split('T')[0];
    }
    this.from = new Date(this.from).toISOString().split('T')[0];
    this.till = new Date(this.till).toISOString().split('T')[0];
    this.http.get<Transaction[]>("/repository/"
      + this.repository_id
      + "/transaction/all?from="
      + this.from + "&till=" + this.till
      + "&account=" + this.account)
      .subscribe(val => this.model = val);
  }

  ngOnInit() {
    this.loadData();
  }

}
