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

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  ngOnInit() {
    this.http.get<Asset>("/repository/" + this.repository_id + "/asset/" + this.asset_id)
      .subscribe(val => this.model = val);
  }

}
