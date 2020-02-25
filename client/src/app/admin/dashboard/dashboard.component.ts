import { Component, OnInit } from '@angular/core';
import { ChartDataSets } from 'chart.js';
import { Color, Label } from 'ng2-charts';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {

  constructor() { }

  lineChartData: ChartDataSets[] = [
    { data: [100, 110, 90, 120, 121, 122, 140, 150, 157], label: 'Pénzeszköz alakulása' },
  ];

  lineChartLabels: Label[] = ["Január", "Február", "Március", "Április", "Május", "Június", "Július", "Augusztus", "Szeptember", "Október", "November", "December"];

  lineChartOptions = {
    responsive: true,
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

  ngOnInit() {
  }

}
