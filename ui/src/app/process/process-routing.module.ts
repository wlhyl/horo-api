import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { ProcessPage } from './process.page';

import { ProcessName } from '../type/enum/process';

import { ProfectionComponent } from './profection/profection.component';
import { ReturnComponent } from './return/return.component';
import { CompareComponent } from './compare/compare.component';
import { FirdariaComponent } from './firdaria/firdaria.component';

const routes: Routes = [
  {
    path: '',
    component: ProcessPage,
  },
  {
    path: ProcessName.path(ProcessName.Profection),
    component: ProfectionComponent,
  },
  {
    path: ProcessName.path(ProcessName.Firdaria),
    component: FirdariaComponent,
  },
  {
    path: ProcessName.path(ProcessName.Transit),
    component: CompareComponent,
    data: { process_name: ProcessName.Transit },
  },
  {
    path: ProcessName.path(ProcessName.SolarcomparNative),
    component: CompareComponent,
    data: { process_name: ProcessName.SolarcomparNative },
  },
  {
    path: ProcessName.path(ProcessName.NativecomparSolar),
    component: CompareComponent,
    data: { process_name: ProcessName.NativecomparSolar },
  },
  {
    path: ProcessName.path(ProcessName.LunarcomparNative),
    component: CompareComponent,
    data: { process_name: ProcessName.LunarcomparNative },
  },
  {
    path: ProcessName.path(ProcessName.NativecomparLunar),
    component: CompareComponent,
    data: { process_name: ProcessName.NativecomparLunar },
  },

  {
    path: ProcessName.path(ProcessName.SolarReturn),
    component: ReturnComponent,
    data: { process_name: ProcessName.SolarReturn },
  },

  {
    path: ProcessName.path(ProcessName.LunarReturn),
    component: ReturnComponent,
    data: { process_name: ProcessName.LunarReturn },
  },

  // { path: Path.Return, component: ReturnComponent },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class ProcessPageRoutingModule {}
