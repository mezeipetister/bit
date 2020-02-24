import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { RoutingModule as AdminRouter } from './routing.module';
import { LayoutComponent } from './layout/layout.component';
import { NavbarComponent } from './partial/navbar/navbar.component';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { ProfileComponent } from './profile/profile.component';
import { ButtonSubmitComponent } from './partial/button-submit/button-submit.component';
import { ErrorDisplayComponent } from './partial/error-display/error-display.component';
import { UserComponent } from './user/user.component';
import { UserNewComponent } from './user/user-new/user-new.component';
import { UserDetailComponent } from './user/user-detail/user-detail.component';
import { RepositoryComponent } from './repository/repository.component';
import { RepositoryDetailComponent } from './repository/repository-detail/repository-detail.component';
import { RepositoryLayoutComponent } from './layout/repository-layout/repository-layout.component';

@NgModule({
  declarations: [
    LayoutComponent,
    NavbarComponent,
    ProfileComponent,
    ButtonSubmitComponent,
    ErrorDisplayComponent,
    UserComponent,
    UserNewComponent,
    UserDetailComponent,
    RepositoryComponent,
    RepositoryDetailComponent,
    RepositoryLayoutComponent,
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    AdminRouter,
  ]
})
export class AdminModule { }
