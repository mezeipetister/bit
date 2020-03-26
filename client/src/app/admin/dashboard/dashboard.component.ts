import { Component, OnInit } from '@angular/core';
import { ChartDataSets, ChartOptions, ChartType } from 'chart.js';
import { Color, Label } from 'ng2-charts';
import { HttpClient } from '@angular/common/http';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { LineChart, Data } from 'src/app/class/chart';
import { concat } from 'rxjs'
import { map } from 'rxjs/operators';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {

  id: string = this.params.hasParam("repository_id");

  stat38: LineChart = new LineChart();
  stat161: LineChart = new LineChart();
  stat5: LineChart = new LineChart();
  stat985: LineChart = new LineChart();
  stat4: LineChart = new LineChart();
  stat468: LineChart = new LineChart();

  constructor(private http: HttpClient, private params: RouterParamService) { }

  cumulateData(val: number[]): number[] {
    val.forEach((value, index) => {
      if (index > 0) {
        value += val[index - 1];
      }
    });
    return val;
  }

  sumArrays(...arrays): number[] {
    const n = arrays.reduce((max, xs) => Math.max(max, xs.length), 0);
    const result = Array.from({ length: n });
    return result.map((_, i) => arrays.map(xs => xs[i] || 0).reduce((sum, x) => sum + x, 0));
  }

  ngOnInit() {
    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 38)
      .subscribe(val => {
        val = this.cumulateData(val);
        this.stat38 = new LineChart('line', [new Data(val, 'Minden 38-as HUF')]);
      });

    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 161)
      .subscribe(val => {
        val = this.cumulateData(val);
        this.stat161 = new LineChart('line', [new Data(val, '161-es HUF')]);
      });

    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 5)
      .subscribe(val => {
        // val = this.cumulateData(val);
        this.stat5 = new LineChart('bar', [new Data(val, 'Minden 5-ös HUF')]);
      });

    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 4)
      .subscribe(val => {
        val = this.cumulateData(val);
        this.stat4 = new LineChart('line', [new Data(val, '4-esek összegezve HUF')]);
      });

    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 468)
      .subscribe(val => {
        // val = this.cumulateData(val);
        this.stat468 = new LineChart('bar', [new Data(val, '468 HUF')]);
      });

    let d9: number[] = [];
    let d8: number[] = [];
    let d5: number[] = [];
    let c9 = this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 9)
      .pipe(map(v => {
        d9 = v.map(x => x * -1);
      }));
    let c8 = this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 8)
      .pipe(map(v => {
        d8 = v;
      }));
    let c5 = this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 5)
      .pipe(map(v => {
        d5 = v;
      }));
    concat(c9, c8, c5).subscribe(v => {
      let data = d9.map((x, index) => x - d8[index] - d5[index]);
      this.stat985 = new LineChart('bar', [new Data(data, '(9*-1)-8-5 HUF')]);
    });
  }

}
