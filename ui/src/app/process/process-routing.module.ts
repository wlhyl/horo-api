import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProcessPage } from './process.page';

import { ProfectionComponent } from './profection/profection.component';
import { TransitComponent } from './transit/transit.component';
const routes: Routes = [
  {
    path: '',
    component: ProcessPage,
  },
  {
    path: 'profection',
    component: ProfectionComponent,
  },
  {
    path: 'transit',
    component: TransitComponent,
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ProcessPageRoutingModule {}
