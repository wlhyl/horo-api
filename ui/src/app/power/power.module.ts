import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { PowerPageRoutingModule } from './power-routing.module';

import { PowerPage } from './power.page';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    PowerPageRoutingModule
  ],
  declarations: [PowerPage]
})
export class PowerPageModule {}
