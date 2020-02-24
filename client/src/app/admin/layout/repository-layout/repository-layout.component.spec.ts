import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RepositoryLayoutComponent } from './repository-layout.component';

describe('RepositoryLayoutComponent', () => {
  let component: RepositoryLayoutComponent;
  let fixture: ComponentFixture<RepositoryLayoutComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RepositoryLayoutComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RepositoryLayoutComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
