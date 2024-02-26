import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { ProcessPageRoutingModule } from './process-routing.module';

import { HoroCommonModule } from '../horo-common/horo-common.module';

import { ProcessPage } from './process.page';
import { ProfectionComponent } from './profection/profection.component';
import { TransitComponent } from './transit/transit.component';
import { ReturnComponent } from './return/return.component';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    ProcessPageRoutingModule,
    HoroCommonModule,
  ],
  declarations: [
    ProcessPage,
    ProfectionComponent,
    TransitComponent,
    ReturnComponent,
  ],
})
export class ProcessPageModule {}
