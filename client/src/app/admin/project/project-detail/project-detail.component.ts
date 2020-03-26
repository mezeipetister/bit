import { Component, OnInit } from '@angular/core';
import { Project } from 'src/app/class/project';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-project-detail',
  templateUrl: './project-detail.component.html',
  styleUrls: ['./project-detail.component.css']
})
export class ProjectDetailComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  project_id: string = this.params.hasParam("project_id");
  model: Project = new Project();

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  ngOnInit() {
    this.http.get<Project>("/repository/" + this.repository_id + "/project/" + this.project_id)
      .subscribe(val => this.model = val);
  }

}
