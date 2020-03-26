import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { Project } from 'src/app/class/project';

@Component({
  selector: 'app-project',
  templateUrl: './project.component.html',
  styleUrls: ['./project.component.css']
})
export class ProjectComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: Project[] = [];

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  ngOnInit() {
    this.http.get<Project[]>("/repository/" + this.repository_id + "/project/all")
      .subscribe(res => this.model = res);
  }

}
