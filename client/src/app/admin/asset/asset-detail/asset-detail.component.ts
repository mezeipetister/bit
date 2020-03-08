import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { Asset } from 'src/app/class/asset';

@Component({
  selector: 'app-asset-detail',
  templateUrl: './asset-detail.component.html',
  styleUrls: ['./asset-detail.component.css']
})
export class AssetDetailComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  asset_id: string = this.params.hasParam("asset_id");
  model: Asset = new Asset();
  current_net_value: number = 0;
  cumulated_depreciation: number = 0;

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  calculateNetValue() {
    this.current_net_value = this.model.value - this.cumulated_depreciation;
  }
  calculateCumulate() {
    this.model.depreciation_monthly.forEach(m => {
      if (new Date(m.month) < new Date()) {
        this.cumulated_depreciation = this.cumulated_depreciation + m.monthly_value;
      }
    });
  }

  ngOnInit() {
    this.http.get<Asset>("/repository/" + this.repository_id + "/asset/" + this.asset_id)
      .subscribe(val => {
        this.model = val;
        this.calculateCumulate();
        this.calculateNetValue();
      });
  }

  remove() {
    this.http.post<Asset>("/repository/" + this.repository_id + "/asset/" + this.asset_id + "/remove", this.model)
      .subscribe(val => this.model = val);
  }
  restore() {
    this.http.post<Asset>("/repository/" + this.repository_id + "/asset/" + this.asset_id + "/restore", this.model)
      .subscribe(val => this.model = val);
  }

  update() {
    if (!this.model.is_active) {
      return;
    }
    this.http.post<Asset>("/repository/" + this.repository_id + "/asset/" + this.asset_id, this.model)
      .subscribe(val => {
        this.model = val;
        alert("Sikeres módosítás!");
      });
  }

}
