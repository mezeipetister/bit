import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { LayoutComponent } from './layout/layout.component';
import { AuthGuard } from '../guard/auth.guard';
import { ProfileComponent } from './profile/profile.component';
import { EmptyComponent } from '../layout/empty/empty.component';
import { UserComponent } from './user/user.component';
import { UserNewComponent } from './user/user-new/user-new.component';
import { UserDetailComponent } from './user/user-detail/user-detail.component';
import { RepositoryComponent } from './repository/repository.component';
import { RepositoryDetailComponent } from './repository/repository-detail/repository-detail.component';
import { RepositoryLayoutComponent } from './layout/repository-layout/repository-layout.component';

const routes: Routes = [
  {
    path: '', component: LayoutComponent, canActivateChild: [AuthGuard], children: [
      {
        path: '', component: RepositoryComponent,
      },
      { path: 'profile', component: ProfileComponent },
      {
        path: 'repository', component: EmptyComponent, children: [
          { path: '', component: RepositoryComponent },
          // { path: 'new', component: UserNewComponent },
          {
            path: ':id', component: RepositoryLayoutComponent, children: [
              {
                path: '', component: RepositoryDetailComponent
              }
            ]
          }
        ]
      },
      {
        path: 'user', component: EmptyComponent, children: [
          { path: '', component: UserComponent },
          { path: 'new', component: UserNewComponent },
          { path: ':id', component: UserDetailComponent }
        ]
      },
      // {
      //   path: 'folder', component: EmptyComponent, children: [
      //     { path: '', component: FolderComponent },
      //     { path: 'new', component: FolderNewComponent },
      //     { path: ':id', component: FolderDetailComponent },
      //     { path: ':id/edit', component: FolderEditComponent },
      //     { path: ':id/new', component: DocumentNewComponent }
      //   ]
      // },
    ]
  },

];

@NgModule({
  imports: [
    RouterModule.forChild(routes)
  ],
  exports: [
    RouterModule
  ]
})
export class RoutingModule { }
