import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

import { Title } from '@angular/platform-browser';
import { ProcessName } from 'src/app/type/enum/process';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';
import {
  HoroscopeCompare,
  ReturnHoroscop,
} from 'src/app/type/interface/respone-data';
import { Canvas } from 'src/app/type/alias/canvas';
import { Horoconfig } from 'src/app/services/config/horo-config.service';
import { Platform } from '@ionic/angular';
import { fabric } from 'fabric';
import { CompareRequest } from 'src/app/type/interface/request-data';
import { lastValueFrom } from 'rxjs';
import { drawAspect, drawHorosco } from 'src/app/utils/image/compare';
import { zoomImage } from 'src/app/utils/image/horo';

@Component({
  selector: 'app-compare',
  templateUrl: './compare.component.html',
  styleUrls: ['./compare.component.scss'],
})
export class CompareComponent implements OnInit {
  horoData = this.storage.horoData;
  processData = this.storage.processData;
  compareData: HoroscopeCompare | null = null;
  returnData: ReturnHoroscop | null = null;

  isAspect = false; // 默认绘制星盘
  // canvas缓存，手机浏览器this.draw()执行慢，因此切换horo、aspect时使用此缓存
  private horoJson: { version: string; objects: Object[] } | undefined =
    undefined;
  private aspectJson: { version: string; objects: Object[] } | undefined =
    undefined;

  private canvas?: Canvas;

  loading = false;

  process_name = ProcessName.Transit;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  get title(): string {
    return ProcessName.name(this.process_name);
  }

  constructor(
    private platform: Platform,
    private route: ActivatedRoute,
    private titleService: Title,
    private api: ApiService,
    private storage: HorostorageService,
    private config: Horoconfig
  ) {}

  async ngOnInit() {
    const process_name = this.route.snapshot.data['process_name'];

    if (process_name === null) {
      this.message = '选择一种比较盘';
      this.isAlertOpen = true;
      return;
    }

    switch (process_name) {
      case ProcessName.Transit:

      case ProcessName.SolarcomparNative:

      case ProcessName.NativecomparSolar:

      case ProcessName.LunarcomparNative:

      case ProcessName.NativecomparLunar:
        this.process_name = process_name;
        break;
      default:
        this.message = `无此种比较盘：${process_name}`;
        this.isAlertOpen = true;
        return;
    }

    this.titleService.setTitle(this.title);

    this.canvas = new fabric.StaticCanvas('canvas');

    await this.drawHoroscope(this.process_name);
  }

  private async drawHoroscope(process_name: ProcessName) {
    this.loading = true;

    try {
      this.compareData = await this.getCopeCompareData(process_name);
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
  private draw() {
    if (this.compareData === null) return;

    this.canvas?.setWidth(0);
    this.canvas?.setHeight(0);
    if (this.isAspect) {
      drawAspect(this.compareData.aspects, this.canvas!, this.config, {
        width: this.config.apsectImage.width,
        heigth: this.config.apsectImage.heigth,
      });
    } else {
      drawHorosco(this.compareData, this.canvas!, this.config, {
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

    await this.drawHoroscope(this.process_name);
  }

  private async getCopeCompareData(
    process_name: ProcessName
  ): Promise<HoroscopeCompare> {
    switch (process_name) {
      case ProcessName.Transit:
        return await this.getTransitData();
      case ProcessName.SolarcomparNative:
        return await this.getReturnComparData(true, true);
      case ProcessName.NativecomparSolar:
        return await this.getReturnComparData(false, true);
      case ProcessName.LunarcomparNative:
        return await this.getReturnComparData(true, false);
      // case ProcessName.NativecomparLunar:
      default:
        return await this.getReturnComparData(false, false);
    }
  }

  private async getTransitData(): Promise<HoroscopeCompare> {
    this.returnData = null;

    const requestData: CompareRequest = {
      native_date: this.horoData.date,
      geo: this.horoData.geo,
      house: this.horoData.house,
      process_date: this.processData.date,
    };

    return await lastValueFrom(this.api.compare(requestData));
  }

  private async getReturnComparData(
    returnComparNative: boolean,
    isSolar: boolean
  ): Promise<HoroscopeCompare> {
    const returnHoroData = isSolar
      ? await this.getSolarReturnData()
      : await this.getLunarReturnData();

    this.returnData = returnHoroData;

    let requestData: CompareRequest = {
      native_date: this.horoData.date,
      geo: this.horoData.geo,
      house: this.horoData.house,
      process_date: this.processData.date,
    };

    if (returnComparNative) {
      requestData.process_date = {
        year: returnHoroData.return_date.year,
        month: returnHoroData.return_date.month,
        day: returnHoroData.return_date.day,
        hour: returnHoroData.return_date.hour,
        minute: returnHoroData.return_date.minute,
        second: returnHoroData.return_date.second,
        tz: returnHoroData.return_date.tz,
        st: false,
      };
    } else {
      requestData.native_date = {
        year: returnHoroData.return_date.year,
        month: returnHoroData.return_date.month,
        day: returnHoroData.return_date.day,
        hour: returnHoroData.return_date.hour,
        minute: returnHoroData.return_date.minute,
        second: returnHoroData.return_date.second,
        tz: returnHoroData.return_date.tz,
        st: false,
      };
      requestData.process_date = this.horoData.date;
    }

    return await lastValueFrom(this.api.compare(requestData));
  }

  private async getSolarReturnData(): Promise<ReturnHoroscop> {
    const requestData: CompareRequest = {
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

    let requestData: CompareRequest = {
      native_date,
      geo: this.horoData.geo,
      house: this.horoData.house,
      process_date: this.processData.date,
    };

    this.horoJson = undefined;
    this.aspectJson = undefined;
    return await lastValueFrom(this.api.lunarReturn(requestData));
  }
}
