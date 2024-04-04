import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';

import { fabric } from 'fabric';
import { Horoscope } from 'src/app/type/interface/response-qizheng';
import { QiZhengRequst } from 'src/app/type/interface/request-data';
import { lastValueFrom } from 'rxjs';
import { drawHorosco } from 'src/app/utils/image/qizheng';
import { QizhengConfigService } from 'src/app/services/config/qizheng-config.service';
import { TipService } from 'src/app/services/qizheng/tip.service';
import { zoomImage } from 'src/app/utils/image/horo';
import { Platform } from '@ionic/angular';

@Component({
  selector: 'app-horo',
  templateUrl: './horo.component.html',
  styleUrls: ['./horo.component.scss'],
})
export class HoroComponent implements OnInit {
  horoData = this.storage.horoData;
  processData = this.storage.processData;

  horoscoData: Horoscope | null = null;

  title = '七政四余';

  private canvas?: fabric.Canvas;

  loading = false;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  constructor(
    private api: ApiService,
    private storage: HorostorageService,
    private config: QizhengConfigService,
    private tip: TipService,
    private titleService: Title,
    private platform: Platform
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);

    this.canvas = new fabric.Canvas('canvas');

    // this.canvas.on('object:scaling', function(e) {
    //   var object = e.target,
    //       scaleX = object?.scaleX,
    //       scaleY = object?.scaleY;

    //   console.log('Object scaling. Object:', object?.get('type'), 'New scaleX:', scaleX, 'New scaleY:', scaleY);
    // });

    this.drawHoroscope();
  }

  private async drawHoroscope() {
    const requestData: QiZhengRequst = {
      native_date: this.horoData.date,
      geo: this.horoData.geo,
      process_date: this.processData.date,
    };

    this.loading = true;

    try {
      this.horoscoData = await lastValueFrom(this.api.qizheng(requestData));
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

  // 绘制星盘
  private draw() {
    if (this.horoscoData === null) return;

    this.canvas?.setWidth(0);
    this.canvas?.setHeight(0);

    drawHorosco(this.horoscoData, this.canvas!, this.config, this.tip, {
      width: this.config.HoroscoImage.width,
      heigth: this.config.HoroscoImage.heigth,
    });

    zoomImage(this.canvas!, this.platform);
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
