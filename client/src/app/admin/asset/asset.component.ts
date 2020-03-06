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

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  ngOnInit() {
    this.http.get<Asset[]>("/repository/" + this.repository_id + "/asset/all")
      .subscribe(val => this.model = val);
  }

}
