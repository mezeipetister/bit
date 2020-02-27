import { Component, OnInit } from '@angular/core';
import { TransactionNew, Transaction } from 'src/app/class/transaction';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-transaction-new',
  templateUrl: './transaction-new.component.html',
  styleUrls: ['./transaction-new.component.css']
})
export class TransactionNewComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: TransactionNew = new TransactionNew();

  helperIsActive: boolean = false;

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  displayHelper() {
    this.helperIsActive = true;
  }

  submit(hasNew: boolean) {
    this.http.put<Transaction>("/repository/" + this.repository_id + "/transaction/new", this.model)
      .subscribe(val => {
        if (hasNew) {
          alert("Hozzáadva!");
          this.model = new TransactionNew();
        } else {
          this.router.navigateByUrl("/repository/" + this.repository_id + "/transaction");
        }
      });
  }
  ngOnInit() {
  }

}
