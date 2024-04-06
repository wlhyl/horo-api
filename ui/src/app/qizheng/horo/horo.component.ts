import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ApiService } from 'src/app/services/api/api.service';
import { HorostorageService } from 'src/app/services/horostorage/horostorage.service';

import {
  DistanceStarLong,
  DongWei,
  Horoscope,
  House,
  Planet,
} from 'src/app/type/interface/response-qizheng';
import { QiZhengRequst } from 'src/app/type/interface/request-data';
import { lastValueFrom } from 'rxjs';
import { Platform } from '@ionic/angular';
import {
  cos,
  degNorm,
  degreeToDMS,
  newtonIteration,
  sin,
} from 'src/app/utils/horo-math';
import { zodiacLong } from 'src/app/utils/qizheng-math';
import { Tip } from 'src/app/type/interface/qizheng';

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

  loading = false;

  isAlertOpen = false;
  alertButtons = ['OK'];
  message = '';

  tip: Tip = {
    message: [],
    show: false,
    x: 0,
    y: 0,
    width: 0,
    height: 0,
  };
  fontSize = 20;

  size = 800;
  get cx() {
    return this.size / 2;
  }
  get cy() {
    return this.cx;
  }

  get ascHouseR() {
    return this.cx * 0.1;
  }

  get zodiacR() {
    return this.cx * 0.2;
  }
  get houseR() {
    return this.cx * 0.3;
  }
  get nativePlanetR() {
    return this.cx * 0.4;
  }
  get xiuR() {
    return this.cx * 0.5;
  }
  get processPlanetR() {
    return this.cx * 0.6;
  }
  get dongWeiR() {
    return this.cx * 0.65;
  }

  get ascHouseText(): string | null {
    if (this.horoscoData === null) return null;

    return `${this.horoscoData.asc_house.xiu}${Math.floor(
      this.horoscoData.asc_house.xiu_degree
    )}度`;
  }

  get zodiacAndHouseLine() {
    return [...Array(12)].map((_, index) => {
      const x0 = this.cx + this.houseR * cos(30 * index);
      const y0 = this.cy - this.houseR * sin(30 * index);

      const x1 = this.cx + this.ascHouseR * cos(30 * index);
      const y1 = this.cy - this.ascHouseR * sin(30 * index);

      return { x0, y0, x1, y1 };
    });
  }

  get zodiacText() {
    return [...Array(12)].map((_, index) => {
      // r1+(r0-r1)/2=(r0+r1)/2
      const x =
        this.cx + ((this.zodiacR + this.ascHouseR) / 2) * cos(30 * index - 15);
      const y =
        this.cy - ((this.zodiacR + this.ascHouseR) / 2) * sin(30 * index - 15);

      const name = this.houseNames[index];
      return { name, x, y };
    });
  }

  get houseText() {
    if (this.horoscoData === null) return [];
    return this.horoscoData.houses.map((house) => {
      const x =
        this.cx + ((this.zodiacR + this.houseR) / 2) * cos(house.long - 15);
      const y =
        this.cy - ((this.zodiacR + this.houseR) / 2) * sin(house.long - 15);

      return { house, x, y };
    });
  }

  get nativePlanetText() {
    if (this.horoscoData === null) return [];

    return this.drawPlanets(
      this.horoscoData.native_planets,
      this.houseR + (this.nativePlanetR - this.houseR) / 3,
      this.houseR + ((this.nativePlanetR - this.houseR) * 2) / 3,
      this.nativePlanetR,
      this.cx,
      this.cy
    );
  }

  get xiuText() {
    if (this.horoscoData === null) return [];

    return this.drawXiu(
      this.horoscoData.distance_star_long,
      (this.nativePlanetR + this.xiuR) / 2,
      this.nativePlanetR,
      this.xiuR,
      this.cx,
      this.cy
    );
  }

  get processPlanetText() {
    if (this.horoscoData === null) return [];

    return this.drawPlanets(
      this.horoscoData.process_planets,
      this.xiuR + ((this.processPlanetR - this.xiuR) * 2) / 3,

      this.xiuR + (this.processPlanetR - this.xiuR) / 3,
      this.xiuR,
      this.cx,
      this.cy
    );
  }
  get dongWeiText() {
    if (this.horoscoData === null) return null;

    return this.drawDongWei(
      this.horoscoData.dong_wei,
      this.processPlanetR,
      this.dongWeiR,
      this.cx,
      this.cy
    );
  }

  private readonly houseNames = [
    '戌',
    '酉',
    '申',
    '未',
    '午',
    '巳',
    '辰',
    '卯',
    '寅',
    '丑',
    '子',
    '亥',
  ];

  ascHouseTip(): Array<string> {
    if (this.horoscoData === null) return [];

    const n = Math.floor(this.horoscoData.asc_house.asc_long / 30);
    const ascLongDMS = degreeToDMS(
      this.horoscoData.asc_house.asc_long - n * 30
    );

    const message = [
      `命度：${this.horoscoData.asc_house.xiu}${Math.floor(
        this.horoscoData.asc_house.xiu_degree
      )}度`,
      `上升：${this.houseNames[n]}宫${ascLongDMS.d
        .toString()
        .padStart(2, '0')}度${ascLongDMS.m
        .toString()
        .padStart(2, '0')}分${ascLongDMS.s.toString().padStart(2, '0')}秒`,
    ];

    return message;
  }

  houseTip(house: House): Array<string> {
    const xiuDMS = degreeToDMS(house.xiu_degree);

    const message = [`${house.xiu}宿：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`];
    return message;
  }

  planetTip(planet: Planet): Array<string> {
    const planetLongOnZoodiac = zodiacLong(planet.long);
    const planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);
    const xiuDMS = degreeToDMS(planet.xiu_degree);

    let message = [
      `${planet.name}`,
      `${planetLongOnZoodiac.zodiac}宫：${planetLongDMSOnZoodiac.d}度${planetLongDMSOnZoodiac.m}分${planetLongDMSOnZoodiac.s}秒`,
      `${planet.xiu}宿：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`,
      `${planet.speed_state}`,
    ];
    if (planet.speed < 0) message[3] = `${message[3]}、逆`;
    else message[3] = `${message[3]}、顺`;

    if (planet.is_stationary) message[3] = `${message[3]}、留`;
    return message;
  }

  xiuTip(xiu: DistanceStarLong, xiuWidth: number): Array<string> {
    // 距星经度
    const planetLongOnZoodiac = zodiacLong(xiu.long);
    const planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);

    // 宿宽度
    const xiuDMS = degreeToDMS(xiuWidth);

    let message = [
      `${xiu.lunar_mansions}`,
      `${planetLongOnZoodiac.zodiac}宫：${planetLongDMSOnZoodiac.d}度${planetLongDMSOnZoodiac.m}分${planetLongDMSOnZoodiac.s}秒`,
      `宿宽：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`,
    ];

    return message;
  }

  dongWeiTip(dong_wei: DongWei): Array<string> {
    // 距星经度
    const planetLongOnZoodiac = zodiacLong(dong_wei.long);
    const planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);

    const xiuDMS = degreeToDMS(dong_wei.xiu_degree);

    const message = [
      `${planetLongOnZoodiac.zodiac}宫：${planetLongDMSOnZoodiac.d}度${planetLongDMSOnZoodiac.m}分${planetLongDMSOnZoodiac.s}秒`,
      `${dong_wei.xiu}宿：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`,
    ];

    return message;
  }

  constructor(
    private api: ApiService,
    private storage: HorostorageService,
    private titleService: Title,
    private platform: Platform
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);

    this.platform.ready().then(() => {
      let width = this.platform.width();
      if (this.size > width) {
        // this.scale = width / this.size;
        this.fontSize = (this.fontSize * width) / this.size;
        this.size = width;

        // console.log(this.size);
        // console.log(this.ascHouseR);
      }
    });

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
    } catch (error: any) {
      const message = error.message + ' ' + error.error.message;
      this.message = message;
      this.isAlertOpen = true;
    } finally {
      this.loading = false;
    }
  }

  private drawPlanets(
    planets: Array<Planet>,
    // 行星文字中心半径
    r: number,
    // 指示线行星侧的半径
    r1: number,
    // 指示线别一侧半径
    r0: number,
    // 圆心
    cx: number,
    cy: number
  ) {
    // 画本命行星
    // 依long从小到大对行星进行排序，方便后面计算绘制位置
    planets.sort((a: Planet, b: Planet) => {
      return degNorm(a.long) - degNorm(b.long);
    });

    // p：行星在canvas的位置
    // 戌宫在-30.0度
    let p = planets.map((x) => degNorm(x.long - 30.0));

    // 以下调整行星间的输出间距，保证行星间到少相距w度
    let w = 12; // 字符间宽度，以角度表示
    for (let i = 0; i < p.length; i++) {
      let n = 0;
      for (let j = 1; j < p.length; j++) {
        if (degNorm(p[(i + j) % p.length] - p[i]) >= w * j) {
          n = j;
          break;
        }
      }
      for (let j = 1; j < n; j++) {
        p[(i + j) % p.length] = degNorm(p[i] + j * w);
      }
    }

    return p.map((long, index) => {
      const x = cx + r * cos(long);
      const y = cy - r * sin(long);

      // 先画行星指示线，以保证行星名字的图层能在指示线之上
      // 标注线半径，本命行星：外圆半径，推运行星：内圆半径
      // const tipR = is_native ? r0 : r1;
      let x0 = x;
      let y0 = y;
      const x1 = cx + r0 * cos(planets[index].long - 30);
      const y1 = cy - r0 * sin(planets[index].long - 30);
      // (x0, y0), (x1, y1)组成的直线方程是：
      // x-x0=(x1-x0)t
      // y-y0=(y1-y0)t
      // 圆心(cx,cy),r=r1 作为标示线的起点
      // 直线与圆交点：
      // ((x1-x0)t+x0-cx)^2+((y1-y0)t+y0-cy)^2=(r1+(r0-r1)*2.5/3)^2
      // 在t=0处作牛顿迭代，求出t>0的值
      const f = (t: number) =>
        ((x1 - x0) * t + x0 - cx) ** 2 +
        ((y1 - y0) * t + y0 - cy) ** 2 -
        r1 ** 2;
      try {
        const t = newtonIteration(0, f);
        x0 = x0 + (x1 - x0) * t;
        y0 = y0 + (y1 - y0) * t;
      } catch (error) {
        console.log(error);
      }

      let color = '#28a745';
      if (planets[index].speed < 0) color = '#dc3545';
      if (planets[index].is_stationary) color = '#ffc107';

      return {
        text: {
          planet: planets[index],
          color,
          x,
          y,
        },
        line: { x0, y0, x1, y1 },
      };
    });
  }

  private drawXiu(
    DistanceStars: Array<DistanceStarLong>,
    // 文字中心半径
    r: number,
    // 内圆半径
    r1: number,
    // 外圆半径
    r0: number,
    cx: number,
    cy: number
  ) {
    return DistanceStars.map((distanceStar, index) => {
      // 画分隔线
      const x0 = cx + (r0 - 6) * cos(distanceStar.long - 30);
      const y0 = cy - (r0 - 6) * sin(distanceStar.long - 30);

      const x1 = cx + (r1 + 4) * cos(distanceStar.long - 30);
      const y1 = cy - (r1 + 4) * sin(distanceStar.long - 30);

      // 画文字
      const nextDistanceStar =
        DistanceStars[(index + 1) % DistanceStars.length];

      // r=r1+(r0-r1)/2=(r0+r1)/2
      // long = long0+(long1-long0)/2=(long0+long1)/2
      // const long = (next_distance_star.long + distance_star.long) / 2;
      const long = degNorm(
        degNorm(nextDistanceStar.long - distanceStar.long) / 2 +
          distanceStar.long
      );
      const x = cx + ((r0 + r1) / 2) * cos(long - 30);
      const y = cy - ((r0 + r1) / 2) * sin(long - 30);

      // 宿宽度
      const xiuWidth = degNorm(nextDistanceStar.long - distanceStar.long);

      return {
        text: { xiu: distanceStar, x, y },
        line: { x0, y0, x1, y1 },
        xiuWidth,
      };
    });
  }

  private drawDongWei(
    dong_wei: DongWei,
    // 内圆半径
    r1: number,
    // 外圆半径
    r0: number,
    cx: number,
    cy: number
  ) {
    let dongWei = [];

    for (let i = 0; i < dong_wei.long_of_per_year.length - 1; i++) {
      // 画分隔线
      const x0 = cx + r0 * cos(dong_wei.long_of_per_year[i] - 30);
      const y0 = cy - r0 * sin(dong_wei.long_of_per_year[i] - 30);

      const x1 = cx + r1 * cos(dong_wei.long_of_per_year[i] - 30);
      const y1 = cy - r1 * sin(dong_wei.long_of_per_year[i] - 30);

      // 画文字
      const next_long =
        dong_wei.long_of_per_year[(i + 1) % dong_wei.long_of_per_year.length];

      // 洞微是逆行，因此用next_long+( long - next_long )/2
      const long = degNorm(
        degNorm(dong_wei.long_of_per_year[i] - next_long) / 2 + next_long
      );

      // r1+(r0-r1)/2=(r0+r1)/2
      const x = cx + ((r0 + r1) / 2) * cos(long - 30);
      const y = cy - ((r0 + r1) / 2) * sin(long - 30);

      dongWei.push({
        line: {
          x0,
          y0,
          x1,
          y1,
        },
        text: { x, y },
      });
    }

    const x0 = cx + r0 * cos(dong_wei.long - 30);
    const y0 = cy - r0 * sin(dong_wei.long - 30);
    const x1 = cx + r1 * cos(dong_wei.long - 30);
    const y1 = cy - r1 * sin(dong_wei.long - 30);

    return {
      dongWeiOfPerYear: dongWei,
      dongWei: { dongWei: dong_wei, x0, y0, x1, y1 },
    };
  }

  showTip(event: MouseEvent, message: Array<string>) {
    this.tip.message = message;
    this.tip.x = event.offsetX;
    this.tip.y = event.offsetY;

    this.tip.show = true;
    const charN = message.map((m) => m.length);
    const n = Math.max(...charN);

    this.tip.width = n * this.fontSize;
    this.tip.height = this.fontSize * message.length;

    if (this.tip.x + this.tip.width > this.size) this.tip.x -= this.tip.width;
    if (this.tip.y + this.tip.height > this.size)
      this.tip.y -= this.tip.y - this.tip.height;
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
