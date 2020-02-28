import { Component, OnInit } from '@angular/core';
import { Router, ActivatedRoute, NavigationEnd, Event } from '@angular/router';

@Component({
  selector: 'app-repository-layout',
  templateUrl: './repository-layout.component.html',
  styleUrls: ['./repository-layout.component.sass']
})
export class RepositoryLayoutComponent implements OnInit {

  bsecond: String = null;
  constructor(private router: Router, private route: ActivatedRoute) {

  }

  ngOnInit() {

  }

}
