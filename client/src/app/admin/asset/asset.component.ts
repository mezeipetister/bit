import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { Asset } from 'src/app/class/asset';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';

@Component({
  selector: 'app-asset',
  templateUrl: './asset.component.html',
  styleUrls: ['./asset.component.css']
})
export class AssetComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: Asset[] = [];
  depreciation_current_year: number = 0;
  depreciation_current_month: number = 0;

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  calculateDepreciationCurrentYear() {
    let thisYear = new Date().getFullYear();
    this.model.forEach(a => {
      let date_activated = new Date(a.date_activated);
      let date_jan1 = new Date(thisYear, 1, 1);
      let date_dec31 = new Date(thisYear, 12, 31);
      a.depreciation_monthly.forEach(m => {
        if (new Date(m.month) >= date_jan1
          && new Date(m.month) <= date_dec31) {
          this.depreciation_current_year = this.depreciation_current_year + m.monthly_value;
        }
      });
    });
  }

  calculateDepreciationCurrentMonth() {
    let thisYear = new Date().getFullYear();
    let thisMonth = new Date().getMonth();
    this.model.forEach(a => {
      a.depreciation_monthly.forEach(m => {
        if (new Date(m.month).getFullYear() == thisYear
          && new Date(m.month).getMonth() == thisMonth) {
          this.depreciation_current_month = this.depreciation_current_month + m.monthly_value;
        }
      });
    });
  }

  ngOnInit() {
    this.http.get<Asset[]>("/repository/" + this.repository_id + "/asset/all")
      .subscribe(val => {
        this.model = val;
        this.calculateDepreciationCurrentYear();
        this.calculateDepreciationCurrentMonth();
      });
  }

}
