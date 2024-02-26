import { Component, OnInit } from '@angular/core';
import { Platform } from '@ionic/angular';
import { ApiService } from 'src/app/services/api/api.service';
import { Horoconfig } from 'src/app/services/config/horo-config.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import { Canvas } from 'src/app/type/alias/canvas';
import { ReturnHoroscop } from 'src/app/type/interface/respone-data';
import { fabric } from 'fabric';
import { lastValueFrom } from 'rxjs';
import { ReturnRequest } from 'src/app/type/interface/request-data';
import { drawAspect, drawReturnHorosco } from 'src/app/utils/image';

@Component({
  selector: 'app-lunar-return',
  templateUrl: './lunar-return.component.html',
  styleUrls: ['./lunar-return.component.scss'],
})
export class LunarReturnComponent implements OnInit {
  horoData = this.storage.horoData;
  processData = this.storage.processData;
  solarReturnData: ReturnHoroscop | null = null;

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

    let native_date = this.horoData.date;

    // 使用日返月亮位置
    if (this.processData.isSolarReturn) {
      // 计算日返
      const requestData: ReturnRequest = {
        native_date: this.horoData.date,
        geo: this.horoData.geo,
        house: this.horoData.house,
        process_date: this.processData.date,
      };

      try {
        const solarReturnData = await lastValueFrom(
          this.api.solarReturn(requestData)
        );

        native_date = {
          year: solarReturnData.return_date.year,
          month: solarReturnData.return_date.month,
          day: solarReturnData.return_date.day,
          hour: solarReturnData.return_date.hour,
          minute: solarReturnData.return_date.minute,
          second: solarReturnData.return_date.second,
          tz: solarReturnData.return_date.tz,
          st: false,
        };
      } catch (error: any) {
        const message = error.message + ' ' + error.error.message;
        this.message = message;
        this.isAlertOpen = true;
        return;
      } finally {
        this.loading = false;
      }
    }

    const requestData: ReturnRequest = {
      native_date,
      geo: this.horoData.geo,
      house: this.horoData.house,
      process_date: this.processData.date,
    };

    try {
      this.solarReturnData = await lastValueFrom(
        this.api.lunarReturn(requestData)
      );
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
    if (this.solarReturnData === null) return;

    this.canvas?.setWidth(0);
    this.canvas?.setHeight(0);
    if (this.isAspect) {
      drawAspect(this.solarReturnData.aspects, this.canvas!, this.config, {
        width: this.apsectImage.width,
        heigth: this.apsectImage.heigth,
      });
    } else {
      drawReturnHorosco(this.solarReturnData, this.canvas!, this.config, {
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
      this.processData.date.year,
      this.processData.date.month - 1,
      this.processData.date.day,
      this.processData.date.hour,
      this.processData.date.minute,
      this.processData.date.second
    );

    date.setFullYear(date.getFullYear() + step.year);
    date.setMonth(date.getMonth() + step.month);
    date.setDate(date.getDate() + step.day);
    date.setHours(date.getHours() + step.hour);
    date.setMinutes(date.getMinutes() + step.minute);
    date.setSeconds(date.getSeconds() + step.second);

    this.processData.date.year = date.getFullYear();
    this.processData.date.month = date.getMonth() + 1;
    this.processData.date.day = date.getDate();
    this.processData.date.hour = date.getHours();
    this.processData.date.minute = date.getMinutes();
    this.processData.date.second = date.getSeconds();

    await this.drawHoroscope();
  }
}
