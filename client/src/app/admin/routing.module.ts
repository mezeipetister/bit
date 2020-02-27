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
      { path: 'profile', component: ProfileComponent, data: { breadcrumb: 'Felhasználói profil' } },
      {
        path: 'repository', component: EmptyComponent, children: [
          { path: '', component: RepositoryComponent },
          { path: 'new', component: RepositoryNewComponent },
          {
            path: ':repository_id', component: RepositoryLayoutComponent, children: [
              // { path: '', component: DashboardComponent },
              { path: '', redirectTo: 'dashboard', pathMatch: 'full' },
              { path: 'dashboard', component: DashboardComponent, data: { breadcrumb: 'Kimutatás' } },
              { path: 'ledger', component: LedgerComponent, data: { breadcrumb: 'Főkönyv' } },
              {
                path: 'transaction', component: EmptyComponent, data: { breadcrumb: 'Tranzakció' }, children: [
                  { path: '', component: TransactionComponent },
                  { path: 'new', component: TransactionNewComponent },
                  { path: ':transaction_id', component: TransactionDetailComponent, }
                ]
              },
              {
                path: 'asset', component: EmptyComponent, data: { breadcrumb: 'Tárgyi eszközök' }, children: [
                  { path: '', component: AssetComponent },
                  { path: 'new', component: AssetNewComponent },
                  { path: ':asset_id', component: AssetDetailComponent }
                ]
              },
              { path: 'project', component: ProjectComponent, data: { breadcrumb: 'Projektek' } },
              {
                path: 'account', component: EmptyComponent, data: { breadcrumb: 'Számlatükör' }, children: [
                  { path: '', component: AccountComponent },
                  { path: 'new', component: AccountNewComponent },
                  { path: ':account_id', component: AccountDetailComponent },
                ]
              },
              { path: 'setting', component: SettingComponent, data: { breadcrumb: 'Beállítások' } },
            ]
          }
        ]
      },
      {
        path: 'user', component: EmptyComponent, children: [
          { path: '', component: UserComponent },
          { path: 'new', component: UserNewComponent },
          { path: ':user_id', component: UserDetailComponent }
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
