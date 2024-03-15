import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class QizhengConfigService {
  readonly fontSize = 20;
  readonly noteTextColor = '#FFFF99';

  // 初始宽、高，绘制完成后会根据屏幕大小缩放
  readonly HoroscoImage = { width: 700, heigth: 700 }; // , fontSize: 20, col: 14, row: 14}

  constructor() {}
}
