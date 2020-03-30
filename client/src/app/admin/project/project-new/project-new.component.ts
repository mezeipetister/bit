import { Component, OnInit } from '@angular/core';
import { Project, ProjectNew } from 'src/app/class/project';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-project-new',
  templateUrl: './project-new.component.html',
  styleUrls: ['./project-new.component.css']
})
export class ProjectNewComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: ProjectNew = new ProjectNew();

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  submit() {
    this.http.put<Project>("/repository/" + this.repository_id + "/project/new", this.model)
      .subscribe(val => this.router.navigateByUrl("/repository/" + this.repository_id + "/project/" + val.id));
  }

  ngOnInit() {
  }

}
