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
import { LedgerComponent } from './ledger/ledger.component';
import { SettingComponent } from './setting/setting.component';
import { AccountComponent } from './account/account.component';
import { ProjectComponent } from './project/project.component';
import { AssetComponent } from './asset/asset.component';
import { TransactionComponent } from './transaction/transaction.component';
import { DashboardComponent } from './dashboard/dashboard.component';
import { AccountDetailComponent } from './account/account-detail/account-detail.component';
import { AccountNewComponent } from './account/account-new/account-new.component';
import { RepositoryNewComponent } from './repository/repository-new/repository-new.component';
import { AssetNewComponent } from './asset/asset-new/asset-new.component';
import { AssetDetailComponent } from './asset/asset-detail/asset-detail.component';
import { TransactionNewComponent } from './transaction/transaction-new/transaction-new.component';
import { TransactionDetailComponent } from './transaction/transaction-detail/transaction-detail.component';

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
          { path: 'new', component: RepositoryNewComponent },
          {
            path: ':id', component: RepositoryLayoutComponent, children: [
              { path: '', redirectTo: './dashboard', pathMatch: 'full' },
              { path: 'dashboard', component: DashboardComponent },
              { path: 'ledger', component: LedgerComponent },
              {
                path: 'transaction', component: EmptyComponent, children: [
                  { path: '', component: TransactionComponent },
                  { path: 'new', component: TransactionNewComponent },
                  { path: ':id', component: TransactionDetailComponent }
                ]
              },
              {
                path: 'asset', component: EmptyComponent, children: [
                  { path: '', component: AssetComponent },
                  { path: 'new', component: AssetNewComponent },
                  { path: ':id', component: AssetDetailComponent }
                ]
              },
              { path: 'project', component: ProjectComponent },
              {
                path: 'account', component: EmptyComponent, children: [
                  { path: '', component: AccountComponent },
                  { path: 'new', component: AccountNewComponent },
                  { path: ':id', component: AccountDetailComponent },
                ]
              },
              { path: 'setting', component: SettingComponent },
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
