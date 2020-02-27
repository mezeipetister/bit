import { Component, OnInit } from '@angular/core';
import { Transaction } from 'src/app/class/transaction';
import { HttpClient } from '@angular/common/http';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { Observable } from 'rxjs';

@Component({
  selector: 'app-transaction-detail',
  templateUrl: './transaction-detail.component.html',
  styleUrls: ['./transaction-detail.component.css']
})
export class TransactionDetailComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  transaction_id: string = this.params.hasParam("transaction_id");

  debitName: string = null;
  creditName: string = null;

  model: Transaction = new Transaction();

  constructor(private http: HttpClient, private params: RouterParamService) { }

  getAccountName(id: string): Observable<Account> {
    return this.http.get<Account>("/repository/" + this.repository_id + "/account/" + id);
  }

  ngOnInit() {
    this.http.get<Transaction>("/repository/" + this.repository_id + "/transaction/" + this.transaction_id)
      .subscribe(val => {
        this.model = val;
        this.getAccountName(val.debit).subscribe(d => this.debitName = d.name);
        this.getAccountName(val.credit).subscribe(c => this.creditName = c.name);
      });
  }

}
