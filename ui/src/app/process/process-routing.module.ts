import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProcessPage } from './process.page';

import { ProfectionComponent } from './profection/profection.component';
import { TransitComponent } from './transit/transit.component';
import { SolarReturnComponent } from './solar-return/solar-return.component';
import { LunarReturnComponent } from './lunar-return/lunar-return.component';

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
  { path: 'solar_return', component: SolarReturnComponent },
  { path: 'lunar_return', component: LunarReturnComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ProcessPageRoutingModule {}
