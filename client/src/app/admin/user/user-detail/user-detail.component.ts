import { Component, OnInit } from '@angular/core';
import { Model } from 'src/app/class/model';
import { Profile } from 'src/app/class/profile';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-user-detail',
  templateUrl: './user-detail.component.html',
  styleUrls: ['./user-detail.component.css']
})
export class UserDetailComponent implements OnInit {

  id: string = this.route.snapshot.paramMap.get("id");
  user: Model<Profile> = new Model<Profile>(this.http, "/user/" + this.id, new Profile());

  constructor(private http: HttpClient, private route: ActivatedRoute, private router: Router) {
  }

  ngOnInit() {
    new Model<Profile>(this.http, "/user/" + this.id, new Profile());
    this.http.get<Profile>("/user/" + this.id).subscribe((val) => this.user.data = val);
  }

}
