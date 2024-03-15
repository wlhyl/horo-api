import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { Path } from './path';

import { QizhengPage } from './qizheng.page';
import { HoroComponent } from './horo/horo.component';

const routes: Routes = [
  {
    path: Path.Input,
    component: QizhengPage,
  },
  {
    path: Path.Horo,
    component: HoroComponent,
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class QizhengPageRoutingModule {}
