import { Component, OnInit } from '@angular/core';
import { RepositoryShort } from 'src/app/class/repository';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-repository',
  templateUrl: './repository.component.html',
  styleUrls: ['./repository.component.css']
})
export class RepositoryComponent implements OnInit {

  constructor(private http: HttpClient) { }

  repositories: RepositoryShort[] = null;

  ngOnInit() {
    this.http.get<RepositoryShort[]>("/repository/all").subscribe(val => {
      this.repositories = val;
      // Set order by date_created asc
      this.repositories.sort((a, b) => a.date_created > b.date_created ? 1 : -1);
    });
  }

}
