import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { IonicModule } from '@ionic/angular';

// 数据双向绑定
import { FormsModule } from '@angular/forms';

import { DateTimeComponent } from './date-time/date-time.component';
import { TimeZoneComponent } from './time-zone/time-zone.component';

import { GeoComponent } from './geo/geo.component';
import { MapComponent } from './geo/map.component';
import { DateTimeChangeComponent } from './date-time-change/date-time-change.component';

@NgModule({
  declarations: [
    DateTimeComponent,
    TimeZoneComponent,
    GeoComponent,
    MapComponent,
    DateTimeChangeComponent,
  ],
  imports: [CommonModule, IonicModule, FormsModule],
  exports: [
    FormsModule,
    DateTimeComponent,
    TimeZoneComponent,
    GeoComponent,
    MapComponent,
    DateTimeChangeComponent,
  ],
})
export class HoroCommonModule {}
