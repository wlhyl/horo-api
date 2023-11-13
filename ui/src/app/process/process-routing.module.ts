import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProcessPage } from './process.page';

import { ProfectionComponent } from './profection/profection.component';

const routes: Routes = [
  {
    path: '',
    component: ProcessPage,
  },
  {
    path: 'profection',
    component: ProfectionComponent,
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ProcessPageRoutingModule {}
