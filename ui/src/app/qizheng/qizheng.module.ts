import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { HoroCommonModule } from '../horo-common/horo-common.module';

import { QizhengPageRoutingModule } from './qizheng-routing.module';

import { QizhengPage } from './qizheng.page';
import { HoroComponent } from './horo/horo.component';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    QizhengPageRoutingModule,
    HoroCommonModule,
  ],
  declarations: [QizhengPage, HoroComponent],
})
export class QizhengPageModule {}
