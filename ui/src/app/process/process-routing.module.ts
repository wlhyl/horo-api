import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProcessPage } from './process.page';

import { Path } from './path';

import { ProfectionComponent } from './profection/profection.component';
import { TransitComponent } from './transit/transit.component';
import { ReturnComponent } from './return/return.component';

const routes: Routes = [
  {
    path: '',
    component: ProcessPage,
  },
  {
    path: Path.Profection,
    component: ProfectionComponent,
  },
  {
    path: Path.Transit,
    component: TransitComponent,
  },
  { path: Path.Return, component: ReturnComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ProcessPageRoutingModule {}
