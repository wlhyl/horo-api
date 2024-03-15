import { Component, OnInit } from '@angular/core';
import { fabric } from 'fabric';
import { Horoscope } from '../type/interface/respone-data';
import { Horoconfig } from '../services/config/horo-config.service';
import { HorostorageService } from '../services/horostorage/horostorage.service';
import { ApiService } from '../services/api/api.service';
import { lastValueFrom } from 'rxjs';
import { Platform } from '@ionic/angular';
import { Canvas } from '../type/alias/canvas';
import { drawAspect, drawHorosco } from '../utils/image/horo';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'teanote-image',
  templateUrl: 'image.component.html',
  styleUrls: ['image.component.scss'],
})
export class ImageComponent implements OnInit {
  public horoData = this.storage.horoData;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  loading = false;

  public horoscoData: Horoscope | null = null;
  isAspect = false; // 默认绘制星盘
  // canvas缓存，手机浏览器this.draw()执行慢，因此切换horo、aspect时使用此缓存
  private horoJson: { version: string; objects: Object[] } | undefined =
    undefined;
  private aspectJson: { version: string; objects: Object[] } | undefined =
    undefined;

  private canvas?: Canvas;

  title = '本命星盘';

  constructor(
    private platform: Platform,
    private api: ApiService,
    private config: Horoconfig,
    private storage: HorostorageService,
    private titleService: Title
  ) {}
  async ngOnInit() {
    this.titleService.setTitle(this.title);

    this.canvas = new fabric.StaticCanvas('canvas');
    await this.drawHoroscope();
  }

  private async drawHoroscope() {
    try {
      this.loading = true;
      this.horoscoData = await lastValueFrom(this.api.getNative(this.horoData));
      this.isAlertOpen = false;
      this.draw();
    } catch (error: any) {
      this.message = error.message + ' ' + error.error.message;
      this.isAlertOpen = true;
    } finally {
      this.loading = false;
    }
  }

  // 绘制星盘和相位
  private draw() {
    if (this.horoscoData === null) return;

    this.canvas?.setWidth(0);
    this.canvas?.setHeight(0);
    if (this.isAspect) {
      drawAspect(this.horoscoData.aspects, this.canvas!, this.config, {
        width: this.config.apsectImage.width,
        heigth: this.config.apsectImage.heigth,
      });
    } else {
      drawHorosco(this.horoscoData, this.canvas!, this.config, {
        width: this.config.HoroscoImage.width,
        heigth: this.config.HoroscoImage.heigth,
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
      this.horoData.date.year,
      this.horoData.date.month - 1,
      this.horoData.date.day,
      this.horoData.date.hour,
      this.horoData.date.minute,
      this.horoData.date.second
    );

    date.setFullYear(date.getFullYear() + step.year);
    date.setMonth(date.getMonth() + step.month);
    date.setDate(date.getDate() + step.day);
    date.setHours(date.getHours() + step.hour);
    date.setMinutes(date.getMinutes() + step.minute);
    date.setSeconds(date.getSeconds() + step.second);

    this.horoData.date.year = date.getFullYear();
    this.horoData.date.month = date.getMonth() + 1;
    this.horoData.date.day = date.getDate();
    this.horoData.date.hour = date.getHours();
    this.horoData.date.minute = date.getMinutes();
    this.horoData.date.second = date.getSeconds();

    await this.drawHoroscope();
  }
}
