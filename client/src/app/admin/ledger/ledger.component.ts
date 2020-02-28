import { Component, OnInit } from '@angular/core';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { HttpClient } from '@angular/common/http';
import { Ledger } from 'src/app/class/ledger';

@Component({
  selector: 'app-ledger',
  templateUrl: './ledger.component.html',
  styleUrls: ['./ledger.component.sass']
})
export class LedgerComponent implements OnInit {

  id: string = this.params.hasParam("repository_id");
  till: string = new Date().toISOString().split('T')[0];
  filter: string = '';
  model: Ledger[] = [];

  constructor(private http: HttpClient, private params: RouterParamService) { }

  load() {
    this.http.get<Ledger[]>("/repository/" + this.id + "/ledger?till=" + this.till)
      .subscribe(val => this.model = val);
  }

  ngOnInit() {
    this.load();
  }

}
