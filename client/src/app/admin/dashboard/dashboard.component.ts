import { Component, OnInit } from '@angular/core';
import { ChartDataSets, ChartOptions, ChartType } from 'chart.js';
import { Color, Label } from 'ng2-charts';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {

  constructor() { }

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

  ngOnInit() {
  }

}
