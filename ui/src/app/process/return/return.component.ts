import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
// import { ReturnPath } from '../path';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import { Horoconfig } from 'src/app/services/config/horo-config.service';
import { Canvas } from 'src/app/type/alias/canvas';
import { fabric } from 'fabric';
import { ReturnHoroscop } from 'src/app/type/interface/respone-data';
import { ReturnRequest } from 'src/app/type/interface/request-data';
import { lastValueFrom } from 'rxjs';
import {
  drawAspect,
  drawReturnHorosco,
  zoomImage,
} from 'src/app/utils/image/horo';
import { Platform } from '@ionic/angular';
import { Title } from '@angular/platform-browser';
import { ProcessName } from 'src/app/type/enum/process';
import { degreeToDMS } from 'src/app/utils/horo-math';
import { QizhengConfigService } from 'src/app/services/config/qizheng-config.service';

@Component({
  selector: 'app-return',
  templateUrl: './return.component.html',
  styleUrls: ['./return.component.scss'],
})
export class ReturnComponent implements OnInit {
  process_name = ProcessName.SolarReturn;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  horoData = this.storage.horoData;
  processData = this.storage.processData;
  rreturnData: ReturnHoroscop | null = null;

  isAspect = false; // 默认绘制星盘
  // canvas缓存，手机浏览器this.draw()执行慢，因此切换horo、aspect时使用此缓存
  private horoJson: { version: string; objects: Object[] } | undefined =
    undefined;
  private aspectJson: { version: string; objects: Object[] } | undefined =
    undefined;

  loading = false;

  private canvas?: Canvas;

  get title(): string {
    return ProcessName.name(this.process_name);
  }

  degreeToDMSFn = degreeToDMS;

  constructor(
    private platform: Platform,
    private route: ActivatedRoute,
    private api: ApiService,
    private storage: HorostorageService,
    public config: Horoconfig,
    private titleService: Title,
    
  ) {}

  async ngOnInit() {
    const process_name = this.route.snapshot.data['process_name'];
    if (process_name === null) {
      this.message = '选择一种返照盘';
      this.isAlertOpen = true;
      return;
    }

    switch (process_name) {
      case ProcessName.SolarReturn:
      case ProcessName.LunarReturn:
        this.process_name = process_name;
        break;
      default:
        this.message = `无此种返照盘：${process_name}`;
        this.isAlertOpen = true;
        return;
    }

    // 设置了this.path再设置title
    this.titleService.setTitle(this.title);

    this.canvas = new fabric.StaticCanvas('canvas');

    await this.drawHoroscope(this.process_name);
  }

  private async getReturnData(
    process_name: ProcessName
  ): Promise<ReturnHoroscop> {
    return process_name == ProcessName.SolarReturn
      ? await this.getSolarReturnData()
      : await this.getLunarReturnData();
  }

  private async getSolarReturnData(): Promise<ReturnHoroscop> {
    const requestData: ReturnRequest = {
      native_date: this.horoData.date,
      geo: this.horoData.geo,
      house: this.horoData.house,
      process_date: this.processData.date,
    };

    return await lastValueFrom(this.api.solarReturn(requestData));
  }

  private async getLunarReturnData(): Promise<ReturnHoroscop> {
    let native_date = this.horoData.date;

    // 使用日返月亮位置
    if (this.processData.isSolarReturn) {
      // 计算日返
      const solarReturnData = await this.getSolarReturnData();

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
    }

    const requestData: ReturnRequest = {
      native_date,
      geo: this.horoData.geo,
      house: this.horoData.house,
      process_date: this.processData.date,
    };

    return await lastValueFrom(this.api.lunarReturn(requestData));
  }

  private async drawHoroscope(process_name: ProcessName) {
    this.loading = true;

    try {
      this.rreturnData = await this.getReturnData(process_name);
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
    if (this.rreturnData === null) return;

    this.canvas?.setWidth(0);
    this.canvas?.setHeight(0);
    if (this.isAspect) {
      drawAspect(this.rreturnData.aspects, this.canvas!, this.config, {
        width: this.config.apsectImage.width,
        heigth: this.config.apsectImage.heigth,
      });
    } else {
      drawReturnHorosco(this.rreturnData, this.canvas!, this.config, {
        width: this.config.HoroscoImage.width,
        heigth: this.config.HoroscoImage.heigth,
      });
    }
    zoomImage(this.canvas!, this.platform);
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

    this.horoJson = undefined;
    this.aspectJson = undefined;
    await this.drawHoroscope(this.process_name);
  }
}
