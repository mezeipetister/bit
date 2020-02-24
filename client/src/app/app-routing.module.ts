import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { NotFoundComponent } from './not-found/not-found.component';
import { EmptyComponent } from './layout/empty/empty.component';
import { LoginComponent } from './login/login/login.component';
import { PasswordresetComponent } from './login/passwordreset/passwordreset.component';

const routes: Routes = [
  {
    path: 'login', component: EmptyComponent, children: [
      { path: '', component: LoginComponent },
      { path: 'elfelejtett-jelszo', component: PasswordresetComponent }
    ]
  },
  { path: '**', component: NotFoundComponent },

];

@NgModule({
  imports: [RouterModule.forRoot(routes, {
    onSameUrlNavigation: 'reload'
  })],
  exports: [RouterModule]
})
export class AppRoutingModule { }
