import { Component, OnInit, ViewChild, Input, ViewChildren } from '@angular/core';
import { TransactionNew, Transaction } from 'src/app/class/transaction';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { Account } from 'src/app/class/account';

@Component({
  selector: 'app-transaction-new',
  templateUrl: './transaction-new.component.html',
  styleUrls: ['./transaction-new.component.css']
})
export class TransactionNewComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: TransactionNew = new TransactionNew();

  helperIsActive: boolean = false;
  accounts: Account[] = [];

  @ViewChildren('subject') subject;

  ngAfterViewInit() {
    this.subject.first.nativeElement.focus();
  }

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  target_is_debit: boolean = true;
  debit_name: string = "";
  credit_name: string = "";

  updateNames() {
    this.debit_name = this.getAccountName(this.model.debit);
    this.credit_name = this.getAccountName(this.model.credit);
  }

  getAccountName(id: string): string {
    let res = null;
    this.accounts.forEach(a => {
      if (a.id == id) {
        res = a.name;
      }
    });
    return res;
  }

  loadAccounts() {
    this.http.get<Account[]>("/repository/" + this.repository_id + "/account/all")
      .subscribe(val => {
        this.accounts = val;
      });
  }

  displayHelper(target_is_debit: boolean = true) {
    this.loadAccounts();
    this.helperIsActive = true;
    this.target_is_debit = target_is_debit;

  }

  submit(hasNew: boolean) {
    if (!confirm('Biztosan hozzáadod?')) {
      return;
    }
    if (this.model.subject.length == 0) {
      alert("A megnevezés mező kötelező!");
      return;
    }
    this.http.put<Transaction>("/repository/" + this.repository_id + "/transaction/new", this.model)
      .subscribe(val => {
        if (hasNew) {
          alert("Hozzáadva!");
          this.model = new TransactionNew();
        } else {
          this.router.navigateByUrl("/repository/" + this.repository_id + "/transaction/" + val.id);
        }
      });
  }
  ngOnInit() {
    this.loadAccounts();
  }

}
