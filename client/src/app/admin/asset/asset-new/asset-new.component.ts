import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { RouterParamService } from 'src/app/services/router-param/router-param.service';
import { AssetNew, Asset } from 'src/app/class/asset';

@Component({
  selector: 'app-asset-new',
  templateUrl: './asset-new.component.html',
  styleUrls: ['./asset-new.component.css']
})
export class AssetNewComponent implements OnInit {

  repository_id: string = this.params.hasParam("repository_id");
  model: AssetNew = new AssetNew();

  constructor(
    private http: HttpClient,
    private router: Router,
    private params: RouterParamService
  ) { }

  submit() {
    this.http.put<Asset>("/repository/" + this.repository_id + "/asset/new", this.model)
      .subscribe(val => this.router.navigateByUrl("/repository/" + this.repository_id + "/asset/" + val.id));
  }

  ngOnInit() {

  }

}
