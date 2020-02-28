import { ChartDataSets, ChartOptions, ChartType } from 'chart.js';
import { Color, Label } from 'ng2-charts';

export class Chart {

}

export class LineChart {
    public data: ChartDataSets[] = [];
    public lineChartColors: Color[] = [];
    public ChartOptions = {
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
    constructor(
        public chartType: string = 'line',
        private dt: number[] = [],
        private dataLabel: string = '',
        public labels: Label[] = [
            "Január", "Február", "Március",
            "Április", "Május", "Június",
            "Július", "Augusztus", "Szeptember",
            "Október", "November", "December"],
        private borderColor: string = '',
        private backgroundColor: string = '',
        public lineChartLegend: boolean = true,
        public lineChartPlugins = [],
    ) {
        this.data = [{ data: this.dt, label: this.dataLabel }];
        this.lineChartColors = [
            {
                borderColor: this.borderColor,
                backgroundColor: this.backgroundColor
            }
        ];
    }
}
