import { Component, Input, OnInit, Output, EventEmitter } from '@angular/core';
import { degreeToDMS } from '../../utils/horo-math';
import { EW, NS } from './enum';

@Component({
  selector: 'horo-geo',
  templateUrl: './geo.component.html',
  styleUrls: ['./geo.component.scss'],
})
export class GeoComponent implements OnInit {
  isModalOpen = false;

  @Output()
  geoLocalNameChange = new EventEmitter<string>();

  @Output()
  geoLongChange = new EventEmitter<number>();

  @Output()
  geoLatChange = new EventEmitter<number>();

  // 地名
  @Input()
  geoLocalName = '';

  // 地理经度
  @Input()
  set geoLong(value: number) {
    const d = degreeToDMS(value);
    this.geoLongD = Math.abs(d.d);
    this.geoLongM = Math.abs(d.m);
    this.geoLongS = Math.abs(d.s);
    this.geoEW = value >= 0 ? EW.E : EW.W;
  }

  get geoLong(): number {
    const long = this.geoLongD + this.geoLongM / 60.0 + this.geoLongS / 3600.0;
    return this.geoEW == EW.E ? long : -long;
  }

  geoLongD = 0;
  geoLongM = 0;
  geoLongS = 0;
  geoEW = EW.E;

  //地理纬度
  @Input()
  set geoLat(value: number) {
    const d = degreeToDMS(value);
    this.geoLatD = Math.abs(d.d);
    this.geoLatM = Math.abs(d.m);
    this.geoLatS = Math.abs(d.s);
    this.geoNS = value >= 0 ? NS.N : NS.S;
  }
  get geoLat(): number {
    const lat = this.geoLatD + this.geoLatM / 60.0 + this.geoLatS / 3600.0;
    return this.geoNS == NS.N ? lat : -lat;
  }

  geoLatD = 0;
  geoLatM = 0;
  geoLatS = 0;
  geoNS = NS.N;

  constructor() {}

  ngOnInit() {}

  ok(): void {
    this.geoLocalNameChange.emit(this.geoLocalName);
    this.geoLongChange.emit(this.geoLong);
    this.geoLatChange.emit(this.geoLat);

    this.isModalOpen = false;
  }
  cancel(): void {
    this.isModalOpen = false;
  }
  open(): void {
    this.isModalOpen = true;
  }
}
