import { Component, OnInit } from '@angular/core';
import { RepositoryNew, RepositoryShort } from 'src/app/class/repository';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';

@Component({
  selector: 'app-repository-new',
  templateUrl: './repository-new.component.html',
  styleUrls: ['./repository-new.component.css']
})
export class RepositoryNewComponent implements OnInit {

  model: RepositoryNew = new RepositoryNew();

  constructor(private http: HttpClient, private router: Router) { }

  submit() {
    this.http.put<RepositoryShort>("/repository/new", this.model)
      .subscribe(val => this.router.navigateByUrl("/repository/" + val.id));
  }

  ngOnInit() {
  }

}
