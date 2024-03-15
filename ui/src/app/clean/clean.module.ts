import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { CleanPageRoutingModule } from './clean-routing.module';

import { CleanPage } from './clean.page';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    CleanPageRoutingModule
  ],
  declarations: [CleanPage]
})
export class CleanPageModule {}
