import { Component, OnInit } from '@angular/core';
import { fabric } from 'fabric';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import { TransitData } from 'src/app/type/interface/request-data';
import { HoroscopeCompare } from 'src/app/type/interface/respone-data';
import { lastValueFrom } from 'rxjs';
import { Canvas } from 'src/app/type/alias/canvas';
import { drawAspect, drawHorosco } from 'src/app/utils/image-compare';
import { Horoconfig } from 'src/app/services/config/horo-config.service';
import { Platform } from '@ionic/angular';

@Component({
  selector: 'app-transit',
  templateUrl: './transit.component.html',
  styleUrls: ['./transit.component.scss'],
})
export class TransitComponent implements OnInit {
  horoData = this.storage.horoData;
  processData = this.storage.processData;
  transitData: HoroscopeCompare | null = null;
  isAspect = false; // 默认绘制星盘
  // canvas缓存，手机浏览器this.draw()执行慢，因此切换horo、aspect时使用此缓存
  private horoJson: { version: string; objects: Object[] } | undefined =
    undefined;
  private aspectJson: { version: string; objects: Object[] } | undefined =
    undefined;

  loading = false;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  private canvas?: Canvas;

  // 初始宽、高，绘制完成后会根据屏幕大小缩放
  private apsectImage = { width: 700, heigth: 700 };
  private HoroscoImage = { width: 700, heigth: 700 }; // , fontSize: 20, col: 14, row: 14}

  constructor(
    private platform: Platform,
    private api: ApiService,
    private storage: HorostorageService,
    private config: Horoconfig
  ) {}

  async ngOnInit() {
    this.canvas = new fabric.StaticCanvas('canvas');

    await this.drawHoroscope();
  }

  private async drawHoroscope() {
    this.loading = true;
    const transitData: TransitData = {
      year: this.horoData.year,
      month: this.horoData.month,
      day: this.horoData.day,
      hour: this.horoData.hour,
      minute: this.horoData.minute,
      second: this.horoData.second,
      tz: this.horoData.tz,
      st: this.horoData.st,

      geo_long: this.horoData.geo_long,
      geo_lat: this.horoData.geo_lat,

      house: this.horoData.house,

      process_year: this.processData.year,
      process_month: this.processData.month,
      process_day: this.processData.day,
      process_hour: this.processData.hour,
      process_minute: this.processData.minute,
      process_second: this.processData.second,
    };

    try {
      this.loading = true;
      this.transitData = await lastValueFrom(this.api.transit(transitData));
      this.isAlertOpen = false;
      this.draw();
    } catch (error: any) {
      const message = error.message + ' ' + error.error.message;
      this.message = message;
      this.isAlertOpen = true;
    } finally {
      this.loading = false;
    }
  }

  // 绘制星盘和相位
  draw() {
    if (this.transitData === null) return;

    this.canvas?.setWidth(0);
    this.canvas?.setHeight(0);
    if (this.isAspect) {
      drawAspect(this.transitData.aspects, this.canvas!, this.config, {
        width: this.apsectImage.width,
        heigth: this.apsectImage.heigth,
      });
    } else {
      drawHorosco(this.transitData, this.canvas!, this.config, {
        width: this.HoroscoImage.width,
        heigth: this.HoroscoImage.heigth,
      });
    }
    this.zoomImage(this.canvas!);
  }

  // 绘制完成后根据屏幕大小缩放
  private zoomImage(canvas: Canvas) {
    this.platform.ready().then(() => {
      let canvasWidth = canvas.getWidth();
      if (!canvasWidth) return;
      let width = this.platform.width();
      let zoom = (width - 10) / canvasWidth;
      if (zoom < 1) {
        canvas.setWidth(width);
        canvas.setHeight(width);
        canvas.setZoom(zoom);
      }
    });
  }

  switchHoroAspect() {
    let json = undefined;
    if (this.isAspect) {
      this.horoJson = this.canvas?.toJSON();
      json = this.aspectJson;
    } else {
      this.aspectJson = this.canvas?.toJSON();
      json = this.horoJson;
    }
    if (json) this.canvas?.loadFromJSON(json, () => {});
    else this.draw();
  }

  async changeStep(step: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    second: number;
  }) {
    let date = new Date(
      this.processData.year,
      this.processData.month - 1,
      this.processData.day,
      this.processData.hour,
      this.processData.minute,
      this.processData.second
    );

    date.setFullYear(date.getFullYear() + step.year);
    date.setMonth(date.getMonth() + step.month);
    date.setDate(date.getDate() + step.day);
    date.setHours(date.getHours() + step.hour);
    date.setMinutes(date.getMinutes() + step.minute);
    date.setSeconds(date.getSeconds() + step.second);

    this.processData.year = date.getFullYear();
    this.processData.month = date.getMonth() + 1;
    this.processData.day = date.getDate();
    this.processData.hour = date.getHours();
    this.processData.minute = date.getMinutes();
    this.processData.second = date.getSeconds();

    await this.drawHoroscope();
  }
}
