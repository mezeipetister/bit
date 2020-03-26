import { Component, OnInit } from '@angular/core';
import { Project } from 'src/app/class/project';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { Transaction, TransactionNew } from 'src/app/class/transaction';

@Component({
  selector: 'app-project-detail',
  templateUrl: './project-detail.component.html',
  styleUrls: ['./project-detail.component.css']
})
export class ProjectDetailComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  project_id: string = this.params.hasParam("project_id");
  model: Project = new Project();
  new_transaction_model: TransactionNew = new TransactionNew();
  helperIsActive: boolean = false;
  accounts: Account[] = [];

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  target_is_debit: boolean = true;
  debit_name: string = "";
  credit_name: string = "";

  updateNames() {
    this.debit_name = this.getAccountName(this.new_transaction_model.debit);
    this.credit_name = this.getAccountName(this.new_transaction_model.credit);
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
    if (this.new_transaction_model.subject.length == 0) {
      alert("A megnevezés mező kötelező!");
      return;
    }
    this.http.put<Transaction>("/repository/" + this.repository_id + "/project/" + this.project_id + "/transaction/new", this.new_transaction_model)
      .subscribe(val => {
        this.new_transaction_model = new TransactionNew();
        this.load_project();
      });
  }
  load_project() {
    this.http.get<Project>("/repository/" + this.repository_id + "/project/" + this.project_id)
      .subscribe(val => this.model = val);
  }
  remove_transaction(transaction_id: number) {
    this.http.post<Transaction[]>("/repository/" + this.repository_id + "/project/" + this.project_id + "/transaction/" + transaction_id + "/remove", [])
      .subscribe(val => this.model.transactions = val);
  }
  project_enable() {
    this.http.post<Project>("/repository/" + this.repository_id + "/project/" + this.project_id + "/enable", [])
      .subscribe(val => this.model = val);
  }
  project_disable() {
    this.http.post<Project>("/repository/" + this.repository_id + "/project/" + this.project_id + "/disable", [])
      .subscribe(val => this.model = val);
  }
  ngOnInit() {
    this.load_project();
    this.loadAccounts();
  }

}
