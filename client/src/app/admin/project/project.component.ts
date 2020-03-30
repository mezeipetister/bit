import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { Project } from 'src/app/class/project';
import { LineChart, Data } from 'src/app/class/chart';

@Component({
  selector: 'app-project',
  templateUrl: './project.component.html',
  styleUrls: ['./project.component.css']
})
export class ProjectComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: Project[] = [];
  stat38: LineChart = new LineChart();
  stat38_plan: LineChart = new LineChart();

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  cumulateData(val: number[]): number[] {
    val.forEach((value, index) => {
      if (index > 0 && value != null) {
        val[index] = value + val[index - 1];
      }
    });
    return val;
  }

  cumulateDataFull(val: number[]): number[] {
    val.forEach((value, index) => {
      if (index > 0) {
        val[index] = value + val[index - 1];
      }
    });
    return val;
  }

  ngOnInit() {
    this.http.get<Project[]>("/repository/" + this.repository_id + "/project/all")
      .subscribe(res => this.model = res);

    this.http.get<number[]>("/repository/" + this.repository_id + "/ledger/stat?account=" + 38)
      .subscribe(val => {
        val = this.cumulateData(val);
        let fact = new Data(val, 'Minden 38-as HUF (TÉNY)');
        this.http.get<number[]>("/repository/" + this.repository_id + "/project/stat?account=" + 38)
          .subscribe(val => {
            val = this.cumulateDataFull(val);
            let plan = new Data(val, 'Minden 38-as HUF (TERV)');
            this.stat38 = new LineChart('line', [plan, fact]);
          });
      });

  }

}
