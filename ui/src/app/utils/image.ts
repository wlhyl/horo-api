import { Platform } from '@ionic/angular';
import { fabric } from 'fabric';

import {
  sin,
  cos,
  degNorm,
  degreeToDMS,
  zodiacLong,
  newtonIteration,
} from './horo-math';
import {
  Aspect,
  Horoscope,
  Planet,
  ReturnHoroscop,
} from '../type/interface/respone-data';
import { Canvas } from '../type/alias/canvas';
import { Horoconfig } from '../services/config/horo-config.service';

/**
 *
 * @param aspects 相位
 * @param aspectCanvas 相位canvas
 * @param options 相位图的相关参数
 * 画相位
 */
export function drawAspect(
  aspects: Array<Aspect>,
  aspectCanvas: Canvas,
  config: Horoconfig,
  options: { width: number; heigth: number }
): void {
  aspectCanvas.clear();
  aspectCanvas.setWidth(options.width);
  aspectCanvas.setHeight(options.heigth);

  const planets = config.horoPlanets;
  const col = planets.length + 1;
  const row = col;
  const width = options.width;
  const heigth = options.heigth;

  // 画网格，共有13颗星，共14行，共14条横线
  // 左侧坐标(width / 14, 0 + i*hegith / 14 )
  // 右侧坐标(width / 14 *(i+1), 0 + i*heigth / 14)
  for (let i = 1; i < col; i++) {
    let x0 = width / row;
    let y0 = (i * heigth) / col;

    let x1 = (width / row) * (i + 1);
    let y1 = y0;

    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      stroke: 'black',
    });
    aspectCanvas.add(path);
    if (i == col - 1) {
      y0 += heigth / col;
      y1 = y0;
      path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
        stroke: 'black',
      });
      aspectCanvas.add(path);
    }
  }

  // 画竖线
  for (let i = 1; i < row; i++) {
    let x0 = ((i + 1) * width) / row;
    let y0 = (i * heigth) / row;

    let x1 = x0;
    let y1 = heigth;

    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      stroke: 'black',
    });
    aspectCanvas.add(path);

    if (i == 1) {
      x0 = width / row;
      x1 = x0;
      path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
        stroke: 'black',
      });
      aspectCanvas.add(path);
    }
  }

  // 画文字
  for (let i = 0; i < planets.length; i++) {
    let planet = planets[i];
    let planetString = config.planetFontString(planet);
    let planetFontFamily = config.planetFontFamily(planet);
    let fontSize = width / col;
    if (planetString.length != 1) fontSize /= planetString.length;
    else fontSize *= 0.8;

    // 画第一列
    // 第一个字符的中心坐标
    // cx=width / col / 2
    // cy = heigth / row * 1.5
    // 第n个字符的中心坐标
    // cx = width / col
    // cy = heigth / row * 1.5 + i * (heigth / row)
    let cx = width / col / 2;
    let cy = (heigth / row) * (i + 1.5);
    let planetText = new fabric.Text(planetString, {
      //绘制文本
      fontSize: fontSize,
      fontFamily: planetFontFamily,
      selectable: false,
    });
    planetText.left = cx - planetText.width! / 2;
    planetText.top = cy - planetText.height! / 2;
    aspectCanvas.add(planetText);

    // 斜列
    // 第一个字符的中心坐标
    // cx=width / col * 1.5
    // cy = heigth / row / * 1.5
    // 第n个字符的中心坐标
    // cx = width / col * 1.5 + i * width / col
    // cy = heigth / row * .5 + i * (heigth / row)
    cx = (width / col) * (1.5 + i);
    cy = (heigth / row) * (i + 0.5);
    planetText = new fabric.Text(planetString, {
      //绘制文本
      fontSize: fontSize,
      fontFamily: planetFontFamily,
      selectable: false,
    });
    planetText.left = cx - planetText.width! / 2;
    planetText.top = cy - planetText.height! / 2;
    aspectCanvas.add(planetText);
  }
  // 画相位
  for (let aspect of aspects) {
    let fontSize = width / col;
    let aspectFontFamily = config.aspectFontFamily();

    // 第一格中心的坐标，即太阳与太阳相交的格子
    // x: width / col + width / col / 2
    // y: heigth / row + heigth / row / 2
    let cx = (width / col) * (1.5 + planets.indexOf(aspect.p0));
    let cy = (heigth / row) * (1.5 + planets.indexOf(aspect.p1));
    let aspectString = config.aspectFontString(aspect.aspect_value);
    let aspectText = new fabric.Text(aspectString, {
      fontSize: fontSize,
      fontFamily: aspectFontFamily,
    });
    aspectText.left = cx - aspectText.width! / 2;
    aspectText.top = cy - aspectText.height! / 2;
    aspectCanvas.add(aspectText);

    // 画相位值
    let aspectValue = degreeToDMS(aspect.d);
    let aspectValueString = '';
    if (aspect.apply) {
      aspectValueString = `${aspectValue.d} A ${aspectValue.m}`;
    } else {
      aspectValueString = `${aspectValue.d} S ${aspectValue.m}`;
    }

    let aspectValueText = new fabric.Text(aspectValueString, {
      fontSize: fontSize * 0.2,
      fontFamily: config.textFont,
    });

    aspectValueText.left = cx - aspectValueText.width! / 2;
    aspectValueText.top = cy + width / col / 2 - aspectValueText.height!;
    aspectCanvas.add(aspectValueText);
  }
}

/**
 * 绘制天宫图
 * @param horosco 天宫图数据
 * @param canvas 天宫图canvas
 * @param options 天宫图图的相关参数
 */
export function drawHorosco(
  horosco: Horoscope,
  canvas: Canvas,
  config: Horoconfig,
  options: { width: number; heigth: number }
) {
  canvas.clear();
  canvas.setWidth(options.width);
  canvas.setHeight(options.heigth);

  // 圆心
  let cx = options.width / 2;
  let cy = options.heigth / 2;
  // 外圆半径
  let r0 = options.width / 2;
  // 内圆半径
  let r1 = r0 - 50;

  drawHouse(horosco.houses_cups, canvas, config, {
    cx: cx,
    cy: cy,
    r0: r0,
    r1: r1,
  });
  // 画行星
  drawPlanets(
    [...horosco.planets, horosco.asc, horosco.mc, horosco.dsc, horosco.ic],
    canvas,
    horosco.houses_cups[0],
    config,
    { cx: cx, cy: cy, r: r1 }
  );

  // 画一些注释文本
  drawNotes(horosco, canvas, config, options);
}

/**
 * 绘制宫头
 * @param cups 宫头度数
 * @param canvas 天宫图canvas
 * @param options 天宫图的相关参数
 */
function drawHouse(
  cups: Array<number>,
  canvas: Canvas,
  config: Horoconfig,
  options: {
    cx: number; // 圆心坐标：x
    cy: number; // 圆心坐标：y
    r0: number; // 外圆半径
    r1: number; // 内圆半径
  }
) {
  const cx = options.cx;
  const cy = options.cy;

  const r0 = options.r0;
  const r1 = options.r1;

  //最外层的圆
  canvas.add(
    new fabric.Circle({
      left: 0,
      top: 0,
      radius: r0,
      // selectable: false,
      fill: '',
      stroke: 'black', //不填充
    })
  );

  //内层的圆
  canvas.add(
    new fabric.Circle({
      left: cx - r1,
      top: cy - r1,
      radius: r1,
      fill: '',
      stroke: 'black', //不填充
    })
  );

  // 画12宫的宫头
  // 以180度起算，因为1宫头在180度方向
  // 所有宫头加上 180 - cups[0].cups，即得图上输出位置
  let cupsOfImage: Array<number> = cups.map((x) => x + 180 - cups[0]);
  for (let i = 0; i < cupsOfImage.length; i++) {
    // 计算并画宫头
    let x = cx + r1 * cos(cupsOfImage[i]);
    let y = cy - r1 * sin(cupsOfImage[i]); // 因纵坐标向下为正，所以用减
    let path = new fabric.Path(`M ${cx}, ${cy} L ${x} ${y}`, {
      stroke: 'black',
    });
    canvas.add(path);

    // 计算并画宫位号，计算文字的圆心
    // d: 宫位宽度，>=0
    let d = degNorm(cupsOfImage[(i + 1) % cupsOfImage.length] - cupsOfImage[i]);
    x = cx + (r1 / 8) * cos(cupsOfImage[i] + d / 2);
    y = cy - (r1 / 8) * sin(cupsOfImage[i] + d / 2);

    let houseNumText = new fabric.Text(`${i + 1}`, {
      fontSize: 10,
      fontFamily: config.textFont,
    });
    houseNumText.left = x - houseNumText.width! / 2;
    houseNumText.top = y - houseNumText.height! / 2;
    canvas.add(houseNumText);

    // 画宫头星座
    let cupsZoodiacLong = zodiacLong(cups[i]);
    x = cx + (r1 + (r0 - r1) / 2) * cos(cupsOfImage[i]);
    y = cy - (r1 + (r0 - r1) / 2) * sin(cupsOfImage[i]);
    let zoodiacText = new fabric.Text(
      `${config.zodiacFontString(cupsZoodiacLong.zodiac)}`,
      {
        fontSize: (r0 - r1) / 2,
        fontFamily: config.zodiacFontFamily(),
      }
    );
    zoodiacText.left = x - zoodiacText.width! / 2;
    zoodiacText.top = y - zoodiacText.height! / 2;
    canvas.add(zoodiacText);

    // 画宫头星座度数
    let cupsZoodiacLongDMS = degreeToDMS(cupsZoodiacLong.long);
    if (i < 7) {
      x = cx + (r1 + (r0 - r1) / 2) * cos(cupsOfImage[i] - 5);
      y = cy - (r1 + (r0 - r1) / 2) * sin(cupsOfImage[i] - 5);
    } else {
      x = cx + (r1 + (r0 - r1) / 2) * cos(cupsOfImage[i] + 5);
      y = cy - (r1 + (r0 - r1) / 2) * sin(cupsOfImage[i] + 5);
    }
    let zoodiacLongDText = new fabric.Text(`${cupsZoodiacLongDMS.d};`, {
      fontSize: (r0 - r1) / 4,
      fontFamily: config.astrologyFont,
    });
    zoodiacLongDText.left = x - zoodiacLongDText.width! / 2;
    zoodiacLongDText.top = y - zoodiacLongDText.height! / 2;
    canvas.add(zoodiacLongDText);

    if (i < 7) {
      x = cx + (r1 + (r0 - r1) / 2) * cos(cupsOfImage[i] + 5);
      y = cy - (r1 + (r0 - r1) / 2) * sin(cupsOfImage[i] + 5);
    } else {
      x = cx + (r1 + (r0 - r1) / 2) * cos(cupsOfImage[i] - 5);
      y = cy - (r1 + (r0 - r1) / 2) * sin(cupsOfImage[i] - 5);
    }
    let zoodiacLongMText = new fabric.Text(`${cupsZoodiacLongDMS.m}'`, {
      fontSize: (r0 - r1) / 4,
      fontFamily: config.astrologyFont,
    });
    zoodiacLongMText.left = x - zoodiacLongMText.width! / 2;
    zoodiacLongMText.top = y - zoodiacLongMText.height! / 2;
    canvas.add(zoodiacLongMText);
  }
}

/**
 * 绘制行星
 * @param planets 行星数组，包含了四轴
 * @param canvas canvas
 * @param firstCupsLong 第一宫头度数
 * @param options (cx, cy)：画布中心坐标，r: 行星字符位置不能超过的半径
 */
function drawPlanets(
  planets: Array<Planet>,
  canvas: Canvas,
  firstCupsLong: number,
  config: Horoconfig,
  options: { cx: number; cy: number; r: number }
) {
  let cx = options.cx;
  let cy = options.cy;

  let r = options.r;
  // 依long从小到大对行星进行排序，方便后面计算绘制位置
  planets.sort((a: Planet, b: Planet) => {
    return degNorm(a.long) - degNorm(b.long);
  });

  // p：行星在canvas的位置
  let p = planets.map((x) => degNorm(x.long + 180 - firstCupsLong));

  // 以下调整行星间的输出间距，保证行星间到少相距w度
  let w = 6; // 字符间宽度，以角度表示
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

  for (let i = 0; i < p.length; i++) {
    // 文字中心位置
    let x = cx + ((r * 8) / 9.0) * cos(p[i]);
    let y = cy - ((r * 8) / 9.0) * sin(p[i]);

    // 先画行星指示线，以保证行星名字的图层能在指示线之上
    let x0 = x;
    let y0 = y;
    const x1 = cx + r * cos(planets[i].long + 180 - firstCupsLong);
    const y1 = cy - r * sin(planets[i].long + 180 - firstCupsLong);
    // (x0, y0), (x1, y1)组成的直线方程是：
    // x-x0=(x1-x0)t
    // y-y0=(y1-y0)t
    // 圆心(cx,cy),r= r*8.5/9.0 作为标示线的起点
    // 直线与圆交点：
    // ((x1-x0)t+x0-cx)^2+((y1-y0)t+y0-cy)^2=(r*8.4/9.0)^2
    // 在t=0处作牛顿迭代，求出t>0的值
    const f = (t: number) =>
      ((x1 - x0) * t + x0 - cx) ** 2 +
      ((y1 - y0) * t + y0 - cy) ** 2 -
      ((r * 8.4) / 9.0) ** 2;
    try {
      const t = newtonIteration(0, f);
      x0 = x0 + (x1 - x0) * t;
      y0 = y0 + (y1 - y0) * t;
    } catch (error) {
      console.log(error);
    }

    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      selectable: false,
      stroke: 'black',
      strokeDashArray: [3, 2], // strokeDashArray[a,b] =》 每隔a个像素空b个像素
    });
    canvas.add(path);

    // 绘制行星名
    let planetString = config.planetFontString(planets[i].name);
    let fontSize = 30;
    if (planetString.length > 1) fontSize = 15;
    let planetText = new fabric.Text(planetString, {
      fontSize: fontSize,
      fontFamily: config.planetFontFamily(planets[i].name),
    });
    planetText.left = x - planetText.width! / 2;
    planetText.top = y - planetText.height! / 2;
    canvas.add(planetText);

    // 画行行星度数
    let planetLongOnZoodiac = zodiacLong(planets[i].long);
    let planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);

    fontSize = 15;
    // 度
    x = cx + ((r * 7) / 9.0) * cos(p[i]);
    y = cy - ((r * 7) / 9.0) * sin(p[i]);
    let planetLongDText = new fabric.Text(`${planetLongDMSOnZoodiac.d};`, {
      fontSize: fontSize,
      fontFamily: config.astrologyFont,
    });
    planetLongDText.left = x - planetLongDText.width! / 2;
    planetLongDText.top = y - planetLongDText.height! / 2;
    canvas.add(planetLongDText);

    // 星座
    x = cx + ((r * 6.5) / 9.0) * cos(p[i]);
    y = cy - ((r * 6.5) / 9.0) * sin(p[i]);

    if (planetString.length > 1) fontSize = 15;
    let zoodiaText = new fabric.Text(
      `${config.zodiacFontString(planetLongOnZoodiac.zodiac)}`,
      {
        fontSize: fontSize,
        fontFamily: config.astrologyFont,
      }
    );
    zoodiaText.left = x - zoodiaText.width! / 2;
    zoodiaText.top = y - zoodiaText.height! / 2;
    canvas.add(zoodiaText);

    // 分
    x = cx + ((r * 6) / 9.0) * cos(p[i]);
    y = cy - ((r * 6) / 9.0) * sin(p[i]);
    let planetLongMText = new fabric.Text(`${planetLongDMSOnZoodiac.m}'`, {
      fontSize: fontSize,
      fontFamily: config.astrologyFont,
    });
    planetLongMText.left = x - planetLongMText.width! / 2;
    planetLongMText.top = y - planetLongMText.height! / 2;
    canvas.add(planetLongMText);

    // 逆
    if (planets[i].speed < 0) {
      x = cx + ((r * 5.5) / 9.0) * cos(p[i]);
      y = cy - ((r * 5.5) / 9.0) * sin(p[i]);
      let planetRetrogradeText = new fabric.Text('>', {
        fontSize: fontSize,
        fontFamily: config.astrologyFont,
      });
      planetRetrogradeText.left = x - planetRetrogradeText.width! / 2;
      planetRetrogradeText.top = y - planetRetrogradeText.height! / 2;
      canvas.add(planetRetrogradeText);
    }
  }
}

// 在左上角绘制说明文字
function drawNotes(
  horosco: Horoscope,
  canvas: Canvas,
  config: Horoconfig,
  options: { width: number; heigth: number }
) {
  const fontSize = 20;

  // 宫位名
  let noteText = new fabric.Text(horosco.house_name, {
    fontSize: fontSize,
    fontFamily: config.textFont,
    top: 0,
    left: 0,
  });
  canvas.add(noteText);

  noteText = new fabric.Text(horosco.is_diurnal ? '白天盘' : '夜间盘', {
    fontSize: fontSize,
    fontFamily: config.textFont,
    top: noteText.height,
    left: 0,
  });
  canvas.add(noteText);

  noteText = new fabric.Text('日主星:', {
    fontSize: fontSize,
    fontFamily: config.textFont,
    top: noteText.top! + noteText.height!,
    left: 0,
  });
  canvas.add(noteText);
  noteText = new fabric.Text(
    `${config.planetFontString(horosco.planetary_day)}`,
    {
      fontSize: fontSize,
      fontFamily: config.planetFontFamily(horosco.planetary_day),
      top: noteText.top,
      left: noteText.width,
    }
  );
  canvas.add(noteText);

  noteText = new fabric.Text('时主星:', {
    fontSize: fontSize,
    fontFamily: config.textFont,
    top: noteText.top! + noteText.height!,
    left: 0,
  });
  canvas.add(noteText);
  noteText = new fabric.Text(
    `${config.planetFontString(horosco.planetary_hours)}`,
    {
      fontSize: fontSize,
      fontFamily: config.planetFontFamily(horosco.planetary_hours),
      top: noteText.top,
      left: noteText.width,
    }
  );
  canvas.add(noteText);
}

/**
 * 绘制返照盘天宫图
 * @param horosco 天宫图数据
 * @param canvas 天宫图canvas
 * @param options 天宫图图的相关参数
 */
export function drawReturnHorosco(
  horosco: ReturnHoroscop,
  canvas: Canvas,
  config: Horoconfig,
  options: { width: number; heigth: number }
) {
  canvas.clear();
  canvas.setWidth(options.width);
  canvas.setHeight(options.heigth);

  // 圆心
  let cx = options.width / 2;
  let cy = options.heigth / 2;
  // 外圆半径
  let r0 = options.width / 2;
  // 内圆半径
  let r1 = r0 - 50;

  drawHouse(horosco.houses_cups, canvas, config, {
    cx: cx,
    cy: cy,
    r0: r0,
    r1: r1,
  });
  // 画行星
  drawPlanets(
    [...horosco.planets, horosco.asc, horosco.mc, horosco.dsc, horosco.ic],
    canvas,
    horosco.houses_cups[0],
    config,
    { cx: cx, cy: cy, r: r1 }
  );
}

// 绘制完成后根据屏幕大小缩放
export function zoomImage(canvas: Canvas, platform: Platform) {
  platform.ready().then(() => {
    let canvasWidth = canvas.getWidth();
    if (!canvasWidth) return;
    let width = platform.width();
    let zoom = (width - 10) / canvasWidth;
    if (zoom < 1) {
      canvas.setWidth(width);
      canvas.setHeight(width);
      canvas.setZoom(zoom);
    }
  });
}
