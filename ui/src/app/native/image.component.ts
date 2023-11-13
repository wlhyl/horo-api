import { Component } from '@angular/core';
import { fabric } from 'fabric';
import { ImageBasic, Canvas } from '../base-component/image-basic/image-basic';
import { Horosco } from '../type/interface/respone-data';
import { Horoconfig } from '../services/config/horo-config.service';
import { HorostorageService } from '../services/horostorage/horostorage.service';
import { ApiService } from '../services/api/api.service';
import { lastValueFrom } from 'rxjs';
import { Platform } from '@ionic/angular';

@Component({
  selector: 'teanote-image',
  templateUrl: 'image.component.html',
  styleUrls: ['image.component.scss'],
})
export class ImageComponent extends ImageBasic {
  public horoData = this.storage.horoData;
  public horosco = {
    error: false,
    message: '',
    loading: false,
  };
  public horoscoData: Horosco | null = null;
  isAspect = false; // 默认绘制星盘
  // canvas缓存，手机浏览器this.draw()执行慢，因此切换horo、aspect时使用此缓存
  private horoJson: { version: string; objects: Object[] } | undefined =
    undefined;
  private aspectJson: { version: string; objects: Object[] } | undefined =
    undefined;

  private canvas?: Canvas;

  // 初始宽、高，绘制完成后会根据屏幕大小缩放
  private apsectImage = { width: 700, heigth: 700 };
  private HoroscoImage = { width: 700, heigth: 700 }; // , fontSize: 20, col: 14, row: 14}

  step = '年';

  constructor(
    private platform: Platform,
    private api: ApiService,
    protected override config: Horoconfig,
    protected storage: HorostorageService
  ) {
    super(config);
  }
  async ngOnInit() {
    this.canvas = new fabric.StaticCanvas('canvas');
    try {
      this.horosco.loading = true;
      this.horoscoData = await this.getHoroscope();
      this.horosco.error = false;
      this.horosco.loading = false;
      this.draw();
    } catch (error: any) {
      this.horosco.message = error.message + ' ' + error.error.message;
      this.horosco.error = true;
      this.horosco.loading = false;
    }
  }

  private async getHoroscope(): Promise<Horosco> {
    // this.horosco.loading = true;

    // let data = {
    //   year: this.horoData.year,
    //   month: this.horoData.month,
    //   day: this.horoData.day,
    //   hour: this.horoData.hour,
    //   minute: this.horoData.minute,
    //   second: this.horoData.second,
    //   tz: this.horoData.tz,
    //   st: this.horoData.st,
    //   geo_long: this.horoData.geo_long,
    //   geo_lat: this.horoData.geo_lat,
    //   house: this.horoData.house,
    // };

    // try {
    return await lastValueFrom(this.api.getNative(this.horoData));
    // } catch (error: any) {
    //   throw error;
    // }
  }

  // 绘制星盘和相位
  draw() {
    if (this.horoscoData)
      this.drawHoroscoAndAspect(this.horoscoData, this.canvas!);
  }

  private drawHoroscoAndAspect(horoscoData: Horosco, canvas: Canvas) {
    canvas.setWidth(0);
    canvas.setHeight(0);
    if (this.isAspect) {
      this.drawAspect(horoscoData.aspects, canvas, {
        width: this.apsectImage.width,
        heigth: this.apsectImage.heigth,
      });
    } else {
      this.drawHorosco(horoscoData, canvas, {
        width: this.HoroscoImage.width,
        heigth: this.HoroscoImage.heigth,
      });
    }
    this.zoomImage(canvas);
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

  changeStep() {
    switch (this.step) {
      case '年':
        this.step = '月';
        break;
      case '月':
        this.step = '日';
        break;
      case '日':
        this.step = '时';
        break;
      case '时':
        this.step = '分';
        break;
      case '分':
        this.step = '秒';
        break;
      default:
        this.step = '年';
    }
  }

  async backDateTime() {
    let date = new Date(
      this.horoData.year,
      this.horoData.month - 1,
      this.horoData.day,
      this.horoData.hour,
      this.horoData.minute,
      this.horoData.second
    );
    switch (this.step) {
      case '年':
        date.setFullYear(date.getFullYear() - 1);
        break;
      case '月':
        date.setMonth(date.getMonth() - 1);
        break;
      case '日':
        date.setDate(date.getDate() - 1);
        break;
      case '时':
        date.setHours(date.getHours() - 1);
        break;
      case '分':
        date.setMinutes(date.getMinutes() - 1);
        break;
      default:
        date.setSeconds(date.getSeconds() - 1);
    }
    this.horoData.year = date.getFullYear();
    this.horoData.month = date.getMonth() + 1;
    this.horoData.day = date.getDate();
    this.horoData.hour = date.getHours();
    this.horoData.minute = date.getMinutes();
    this.horoData.second = date.getSeconds();
    try {
      this.horosco.loading = true;
      this.horoscoData = await this.getHoroscope();
      this.horosco.error = false;
      this.horosco.loading = false;
    } catch (error: any) {
      this.horosco.message = error.message + ' ' + error.error.message;
      this.horosco.error = true;
      this.horosco.loading = false;
    }
    this.aspectJson = undefined;
    this.horoJson = undefined;
    this.draw();
  }

  async forwardDateTime() {
    let date = new Date(
      this.horoData.year,
      this.horoData.month - 1,
      this.horoData.day,
      this.horoData.hour,
      this.horoData.minute,
      this.horoData.second
    );
    switch (this.step) {
      case '年':
        date.setFullYear(date.getFullYear() + 1);
        break;
      case '月':
        date.setMonth(date.getMonth() + 1);
        break;
      case '日':
        date.setDate(date.getDate() + 1);
        break;
      case '时':
        date.setHours(date.getHours() + 1);
        break;
      case '分':
        date.setMinutes(date.getMinutes() + 1);
        break;
      default:
        date.setSeconds(date.getSeconds() + 1);
    }
    this.horoData.year = date.getFullYear();
    this.horoData.month = date.getMonth() + 1;
    this.horoData.day = date.getDate();
    this.horoData.hour = date.getHours();
    this.horoData.minute = date.getMinutes();
    this.horoData.second = date.getSeconds();
    try {
      this.horosco.loading = true;
      this.horoscoData = await this.getHoroscope();
      this.horosco.error = false;
      this.horosco.loading = false;
    } catch (error: any) {
      this.horosco.message = error.message + ' ' + error.error.message;
      this.horosco.error = true;
      this.horosco.loading = false;
    }
    this.aspectJson = undefined;
    this.horoJson = undefined;
    this.draw();
  }
}
