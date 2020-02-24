import { Component, OnInit } from '@angular/core';
import { Profile, ProfileNew } from 'src/app/class/profile';
import { Model } from 'src/app/class/model';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { HttpError } from 'src/app/class/http-error';

@Component({
  selector: 'app-user-new',
  templateUrl: './user-new.component.html',
  styleUrls: ['./user-new.component.css']
})
export class UserNewComponent implements OnInit {

  user: Model<ProfileNew> = new Model<ProfileNew>(this.http, "/user/new", new ProfileNew());

  // submit() {
  //   this.http.post<ProfileNew>("/user/new", this.user.data).subscribe((user) => {
  //     this.router.navigateByUrl('./a/user/' + user.username);
  //   }, (err) => this.e.error = err)
  // }

  constructor(private http: HttpClient, private router: Router) { }

  ngOnInit() {
  }

}
