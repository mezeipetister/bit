import { Component, OnInit } from '@angular/core';
import { ChartDataSets, ChartOptions, ChartType } from 'chart.js';
import { Color, Label } from 'ng2-charts';
import { HttpClient } from '@angular/common/http';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { LineChart } from 'src/app/class/chart';

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

  constructor(private http: HttpClient, private params: RouterParamService) { }

  barChartOptions: ChartOptions = {
    responsive: true,
  };
  barChartLabels: Label[] = ["Január", "Február", "Március", "Április", "Május", "Június", "Július", "Augusztus", "Szeptember", "Október", "November", "December"];
  barChartType: ChartType = 'bar';
  barChartLegend = true;
  barChartPlugins = [];

  barChartData: ChartDataSets[] = [
    { data: [15132000, 16000000, 16500000, 5000000], label: 'Beruházások' }
  ];

  lineChartData: ChartDataSets[] = [
    { data: [100, 110, 90, 120, 121, 122, 140, 150, 157], label: 'Pénzeszköz HUF' },
  ];

  lineChartLabels: Label[] = ["Január", "Február", "Március", "Április", "Május", "Június", "Július", "Augusztus", "Szeptember", "Október", "November", "December"];

  ChartOptions = {
    responsive: true,
    tooltips: {
      callbacks: {
        label: function (tooltipItems, data) {
          return data.labels[tooltipItems.index] + '' + data.datasets[0].data[tooltipItems.index].toLocaleString();
        }
      }
    },
    scales: {
      yAxes: [{
        ticks: {
          beginAtZero: false,
          callback: function (value, index, values) {
            return value.toLocaleString();
          }
        }
      }]
    }
  };

  lineChartColors: Color[] = [
    {
      // borderColor: 'black',
      // backgroundColor: 'rgba(255,255,0,0.28)',
    },
  ];

  lineChartLegend = true;
  lineChartPlugins = [];
  lineChartType = 'line';

  cumulateData(val: number[]): number[] {
    val[1] = val[0] + val[1];
    val[2] = val[1] + val[2];
    val[3] = val[2] + val[3];
    val[4] = val[3] + val[4];
    val[5] = val[4] + val[5];
    val[6] = val[5] + val[6];
    val[7] = val[6] + val[7];
    val[8] = val[7] + val[8];
    val[9] = val[8] + val[9];
    val[10] = val[9] + val[10];
    val[11] = val[10] + val[11];
    return val;
  }

  ngOnInit() {
    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 38)
      .subscribe(val => {
        val = this.cumulateData(val);
        this.stat38 = new LineChart('line', val, 'Pénzeszköz HUF');
      });

    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 161)
      .subscribe(val => {
        val = this.cumulateData(val);
        this.stat161 = new LineChart('line', val, 'Beruházások HUF');
      });

    this.http.get<number[]>("/repository/" + this.id + "/ledger/stat?account=" + 5)
      .subscribe(val => {
        val = this.cumulateData(val);
        this.stat5 = new LineChart('line', val, 'Költségek HUF');
      });
  }

}
