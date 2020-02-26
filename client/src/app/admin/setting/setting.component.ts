import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router, ActivatedRoute } from '@angular/router';
import { RepositoryNew, RepositoryShort } from 'src/app/class/repository';

@Component({
  selector: 'app-setting',
  templateUrl: './setting.component.html',
  styleUrls: ['./setting.component.css']
})
export class SettingComponent implements OnInit {

  id: string = this.route.root.firstChild.firstChild.firstChild.snapshot.paramMap.get("id");
  model: RepositoryNew = new RepositoryNew();

  constructor(private http: HttpClient,
    private route: ActivatedRoute,
    private router: Router) { }

  submit() {
    this.http.post<RepositoryShort>("/repository/" + this.id, this.model)
      .subscribe(val => {
        this.model.name = val.name;
        this.model.description = val.description;
        alert("Sikeresen mentve");
      });
  }

  remove() {
    if (confirm("Biztosan törlöd?")) {
      this.http.post<RepositoryShort>("/repository/" + this.id + "/remove", [])
        .subscribe(val => this.router.navigateByUrl("/"));
    }
  }

  ngOnInit() {
    this.http.get<RepositoryShort>("/repository/" + this.id)
      .subscribe(val => {
        this.model.name = val.name;
        this.model.description = val.description;
      });
  }

}
