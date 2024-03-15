import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { CleanPage } from './clean.page';

const routes: Routes = [
  {
    path: '',
    component: CleanPage
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class CleanPageRoutingModule {}
