import {
  ASCHouse,
  DistanceStarLong,
  DongWei,
  Horoscope,
  House,
  Planet,
} from 'src/app/type/interface/response-qizheng';
import { fabric } from 'fabric';
import { QizhengConfigService } from 'src/app/services/config/qizheng-config.service';
import { cos, degNorm, degreeToDMS, newtonIteration, sin } from '../horo-math';
import { TipService } from 'src/app/services/qizheng/tip.service';
import { zodiacLong } from '../qizheng-math';

/**
 * 绘制天宫图
 * @param horosco 天宫图数据
 * @param canvas 天宫图canvas
 * @param options 天宫图图的相关参数
 */
export function drawHorosco(
  horosco: Horoscope,
  canvas: fabric.Canvas,
  config: QizhengConfigService,
  tip: TipService,
  options: { width: number; heigth: number }
) {
  canvas.clear();
  canvas.setWidth(options.width);
  canvas.setHeight(options.heigth);

  // 圆心
  const cx = options.width / 2;
  const cy = options.heigth / 2;
  // 外圆半径
  const r = options.width / 2;

  // 画圆
  for (let i = 1; i < 7; i++) {
    const r1 = (r / 9) * i;
    canvas.add(
      new fabric.Circle({
        left: cx - r1,
        top: cy - r1,
        radius: r1,
        fill: '',
        stroke: 'black', //不填充
        selectable: true,
      })
    );
  }

  // 洞微圆
  const dongWeiR = (r / 9) * 6.5;
  canvas.add(
    new fabric.Circle({
      left: cx - dongWeiR,
      top: cy - dongWeiR,
      radius: dongWeiR,
      fill: '',
      stroke: 'black', //不填充
      selectable: true,
    })
  );

  drawASCHouse(horosco.asc_house, canvas, config, tip, {
    cx: cx,
    cy: cy,
    r: r / 9.0,
  });

  drawZodiac(canvas, config, {
    cx: cx,
    cy: cy,
    r0: (r * 2.0) / 9.0,
    r1: r / 9.0,
  });

  drawHouse(horosco.houses, canvas, config, tip, {
    cx: cx,
    cy: cy,
    r0: (r * 3.0) / 9.0,
    r1: (r * 2.0) / 9.0,
  });

  // 画本命行星
  drawPlanets(
    horosco.native_planets,
    canvas,
    tip,
    config,
    {
      cx: cx,
      cy: cy,
      r0: (r * 4.0) / 9.0,
      r1: (r * 3.0) / 9.0,
    },
    true
  );

  drawDistanceStar(horosco.distance_star_long, canvas, tip, config, {
    cx: cx,
    cy: cy,
    r0: (r * 5.0) / 9.0,
    r1: (r * 4.0) / 9.0,
  });

  // 画推运行星
  drawPlanets(
    horosco.process_planets,
    canvas,
    tip,
    config,
    {
      cx: cx,
      cy: cy,
      r0: (r * 6.0) / 9.0,
      r1: (r * 5.0) / 9.0,
    },
    false
  );

  // 画洞微
  drawDongWei(horosco.dong_wei, canvas, tip, config, {
    cx: cx,
    cy: cy,
    r0: dongWeiR,
    r1: (r * 6.0) / 9.0,
  });

  const group = new fabric.Group(canvas.getObjects(), {
    selectable: true,
    subTargetCheck: true,

    // perPixelTargetFind: true,
  });

  // https://blog.csdn.net/beauty_600/article/details/129621372
  group.setControlsVisibility({
    ml: false, // 左中
    mt: false,
    mr: false,
    mb: false,
    mtr: false, // 旋转
    // bl;: false, 左下
    // bl:false // 右下
    // tl: false // 左上
    // tr: false // 右上
  });

  // group.hasRotatingPoint=false

  // canvas.getObjects().forEach(o=>canvas.remove(o))
  canvas.clear();
  canvas.add(group);

  // 画农历
  const nativeLunarCalendarText = `${
    horosco.native_lunar_calendar.lunar_year_gan_zhi[0]
  } ${horosco.native_lunar_calendar.lunar_month_gan_zhi[0]} ${
    horosco.native_lunar_calendar.lunar_day_gan_zhi[0]
  } ${horosco.native_lunar_calendar.time_gan_zhi[0]}
${horosco.native_lunar_calendar.lunar_year_gan_zhi[1]} ${
    horosco.native_lunar_calendar.lunar_month_gan_zhi[1]
  } ${horosco.native_lunar_calendar.lunar_day_gan_zhi[1]} ${
    horosco.native_lunar_calendar.time_gan_zhi[1]
  }
${horosco.native_lunar_calendar.solar_term_first.name}：${
    horosco.native_lunar_calendar.solar_term_first.year
  }-${horosco.native_lunar_calendar.solar_term_first.month
    .toString()
    .padStart(2, '0')}-${horosco.native_lunar_calendar.solar_term_first.day
    .toString()
    .padStart(2, '0')} ${horosco.native_lunar_calendar.solar_term_first.hour
    .toString()
    .padStart(2, '0')}:${horosco.native_lunar_calendar.solar_term_first.minute
    .toString()
    .padStart(2, '0')}:${horosco.native_lunar_calendar.solar_term_first.second
    .toString()
    .padStart(2, '0')}
${horosco.native_lunar_calendar.solar_term_second.name}：${
    horosco.native_lunar_calendar.solar_term_second.year
  }-${horosco.native_lunar_calendar.solar_term_second.month
    .toString()
    .padStart(2, '0')}-${horosco.native_lunar_calendar.solar_term_second.day
    .toString()
    .padStart(2, '0')} ${horosco.native_lunar_calendar.solar_term_second.hour
    .toString()
    .padStart(2, '0')}:${horosco.native_lunar_calendar.solar_term_second.minute
    .toString()
    .padStart(2, '0')}:${horosco.native_lunar_calendar.solar_term_second.second
    .toString()
    .padStart(2, '0')}
农历；${horosco.native_lunar_calendar.lunar_year}年${
    horosco.native_lunar_calendar.lunar_month
  }${horosco.native_lunar_calendar.lunar_day}
    `;
  const nativeLunarCalendarTextCanvas = new fabric.Text(
    nativeLunarCalendarText,
    {
      fontSize: (config.fontSize * 2) / 3,
      // fontFamily: config.textFont,
      selectable: false,
    }
  );

  nativeLunarCalendarTextCanvas.left = 0;
  nativeLunarCalendarTextCanvas.top = 0;
  canvas.add(nativeLunarCalendarTextCanvas);

  const processLunarCalendarText = `${
    horosco.process_lunar_calendar.lunar_year_gan_zhi[0]
  } ${horosco.process_lunar_calendar.lunar_month_gan_zhi[0]} ${
    horosco.process_lunar_calendar.lunar_day_gan_zhi[0]
  } ${horosco.process_lunar_calendar.time_gan_zhi[0]}
${horosco.process_lunar_calendar.lunar_year_gan_zhi[1]} ${
    horosco.process_lunar_calendar.lunar_month_gan_zhi[1]
  } ${horosco.process_lunar_calendar.lunar_day_gan_zhi[1]} ${
    horosco.process_lunar_calendar.time_gan_zhi[1]
  }
${horosco.process_lunar_calendar.solar_term_first.name}：${
    horosco.process_lunar_calendar.solar_term_first.year
  }-${horosco.process_lunar_calendar.solar_term_first.month
    .toString()
    .padStart(2, '0')}-${horosco.process_lunar_calendar.solar_term_first.day
    .toString()
    .padStart(2, '0')} ${horosco.process_lunar_calendar.solar_term_first.hour
    .toString()
    .padStart(2, '0')}:${horosco.process_lunar_calendar.solar_term_first.minute
    .toString()
    .padStart(2, '0')}:${horosco.process_lunar_calendar.solar_term_first.second
    .toString()
    .padStart(2, '0')}
${horosco.process_lunar_calendar.solar_term_second.name}：${
    horosco.process_lunar_calendar.solar_term_second.year
  }-${horosco.process_lunar_calendar.solar_term_second.month
    .toString()
    .padStart(2, '0')}-${horosco.process_lunar_calendar.solar_term_second.day
    .toString()
    .padStart(2, '0')} ${horosco.process_lunar_calendar.solar_term_second.hour
    .toString()
    .padStart(2, '0')}:${horosco.process_lunar_calendar.solar_term_second.minute
    .toString()
    .padStart(2, '0')}:${horosco.process_lunar_calendar.solar_term_second.second
    .toString()
    .padStart(2, '0')}
农历；${horosco.process_lunar_calendar.lunar_year}年${
    horosco.process_lunar_calendar.lunar_month
  }${horosco.process_lunar_calendar.lunar_day}
    `;
  const processLunarCalendarTextCanvas = new fabric.Text(
    processLunarCalendarText,
    {
      fontSize: (config.fontSize * 2) / 3,
      // fontFamily: config.textFont,
      selectable: false,
    }
  );

  processLunarCalendarTextCanvas.left =
    canvas.width! - processLunarCalendarTextCanvas.width!;
  processLunarCalendarTextCanvas.top = 0;
  canvas.add(processLunarCalendarTextCanvas);
}

/**
 * 绘制上升宫标注
 * @param asc_house  上升宫
 * @param canvas 天宫图canvas
 * @param options 天宫图的相关参数
 */
function drawASCHouse(
  asc_house: ASCHouse,
  canvas: fabric.Canvas,
  config: QizhengConfigService,
  tip: TipService,
  options: {
    cx: number; // 圆心坐标：x
    cy: number; // 圆心坐标：y
    r: number; // 标注圆半径
  }
) {
  const cx = options.cx;
  const cy = options.cy;

  const r = options.r;

  const group = new fabric.Group(undefined, {
    selectable: true,
    subTargetCheck: true,
    // perPixelTargetFind: true,
  });

  const houseNames = [
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
  const n = Math.floor(asc_house.asc_long / 30);
  const ascLongDMS = degreeToDMS(asc_house.asc_long - n * 30);

  const message = `命度：${asc_house.xiu}${Math.floor(
    asc_house.xiu_degree
  )}度\n上升：${houseNames[n]}宫${ascLongDMS.d}度${ascLongDMS.m}分${
    ascLongDMS.s
  }秒`;

  const houseNumText = new fabric.Text(
    `${asc_house.xiu}${Math.floor(asc_house.xiu_degree)}度`,
    {
      fontSize: config.fontSize,
      // fontFamily: config.textFont,
      selectable: true,
    }
  );

  houseNumText.left = cx - houseNumText.width! / 2;
  houseNumText.top = cy - houseNumText.height! / 2;

  // 以下一行不能放到 mousedown的回调函数中，因为houseNumText添加到组后会
  // 重新计算坐标，此坐标不再是相对于整个画布
  const noteText = tip.newTip(message, houseNumText, canvas);

  houseNumText.on('mousedown', (e) => {
    tip.showTip(noteText, canvas);
  });
  group.addWithUpdate(houseNumText);
  canvas.add(group);
}

/**
 * 绘制黄道12宫
 * @param canvas 天宫图canvas
 * @param options 天宫图的相关参数
 */
function drawZodiac(
  canvas: fabric.Canvas,
  config: QizhengConfigService,
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

  const houseNames = [
    '酉', // 酉宫从0度起
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
    '戌',
  ];

  houseNames.forEach((houseName, index) => {
    // 画宫位分隔线
    const x0 = cx + r1 * cos(30 * index);
    const y0 = cx - r1 * sin(30 * index);

    const x1 = cx + r0 * cos(30 * index);
    const y1 = cx - r0 * sin(30 * index);
    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      stroke: 'black',
      selectable: true,
    });
    canvas.add(path);

    // 画文字
    // r1+(r0-r1)/2=r0/2+r1/2=(r0+r1)/2
    const x = cx + ((r0 + r1) / 2) * cos(30 * index + 15);
    const y = cx - ((r0 + r1) / 2) * sin(30 * index + 15);

    let houseNumText = new fabric.Text(`${houseName}`, {
      fontSize: config.fontSize,
      selectable: true,
      // fontFamily: config.textFont,
    });
    houseNumText.left = x - houseNumText.width! / 2;
    houseNumText.top = y - houseNumText.height! / 2;
    canvas.add(houseNumText);
  });
}

/**
 * 绘制宫位
 * @param houses 宫位
 * @param canvas 天宫图canvas
 * @param options 天宫图的相关参数
 */
function drawHouse(
  houses: Array<House>,
  canvas: fabric.Canvas,
  config: QizhengConfigService,
  tip: TipService,
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

  houses.forEach((house, index) => {
    // 画宫位分隔线
    const x0 = cx + r1 * cos(30 * index);
    const y0 = cx - r1 * sin(30 * index);

    const x1 = cx + r0 * cos(30 * index);
    const y1 = cx - r0 * sin(30 * index);
    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      stroke: 'black',
      selectable: true,
    });
    canvas.add(path);

    // 画文字
    // r1+(r0-r1)/2=r0/2+r1/2=(r0+r1)/2
    // 因为图上0度从酉开始，因需要减去30
    const x = cx + ((r0 + r1) / 2) * cos(house.long + 15 - 30);
    const y = cx - ((r0 + r1) / 2) * sin(house.long + 15 - 30);

    const xiuDMS = degreeToDMS(house.xiu_degree);
    // const noteText = new fabric.Text(
    const message = `${house.xiu}宿：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`;

    const houseNumText = new fabric.Text(`${house.name}`, {
      fontSize: config.fontSize,
      selectable: true,
      // fontFamily: config.textFont,
    });
    houseNumText.left = x - houseNumText.width! / 2;
    houseNumText.top = y - houseNumText.height! / 2;

    const noteText = tip.newTip(message, houseNumText, canvas);

    houseNumText.on('mousedown', (e) => {
      tip.showTip(noteText, canvas);
    });
    canvas.add(houseNumText);
  });
}

/**
 * 绘制本命盘行星
 * @param planets 行星数组，包含了四轴
 * @param canvas canvas
 * @param firstCupsLong 第一宫头度数
 * @param options (cx, cy)：画布中心坐标，r: 行星字符位置不能超过的半径
 */
function drawPlanets(
  planets: Array<Planet>,
  canvas: fabric.Canvas,
  tip: TipService,
  config: QizhengConfigService,
  options: {
    cx: number; // 圆心坐标：x
    cy: number; // 圆心坐标：y
    r0: number; // 外圆半径
    r1: number; // 内圆半径
  },
  is_native: boolean
) {
  const cx = options.cx;
  const cy = options.cy;

  const r0 = options.r0;
  const r1 = options.r1;
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

  for (let i = 0; i < p.length; i++) {
    // 文字中心位置
    // r1+(r0-r1)/2 = (r0+r1)/2
    const x = cx + ((r0 + r1) / 2) * cos(p[i]);
    const y = cy - ((r0 + r1) / 2) * sin(p[i]);

    // 先画行星指示线，以保证行星名字的图层能在指示线之上
    // 标注线半径，本命行星：外圆半径，推运行星：内圆半径
    // const tipR = is_native ? r0 : r1;
    let x0 = x;
    let y0 = y;
    const x1 = cx + (is_native ? r0 : r1) * cos(planets[i].long - 30);
    const y1 = cy - (is_native ? r0 : r1) * sin(planets[i].long - 30);
    // (x0, y0), (x1, y1)组成的直线方程是：
    // x-x0=(x1-x0)t
    // y-y0=(y1-y0)t
    // 圆心(cx,cy),r= r1+(r0-r1)*2.5/3 作为标示线的起点
    // 直线与圆交点：
    // ((x1-x0)t+x0-cx)^2+((y1-y0)t+y0-cy)^2=(r1+(r0-r1)*2.5/3)^2
    // 在t=0处作牛顿迭代，求出t>0的值
    const f = (t: number) =>
      ((x1 - x0) * t + x0 - cx) ** 2 +
      ((y1 - y0) * t + y0 - cy) ** 2 -
      (r1 + ((r0 - r1) * (is_native ? 2.5 : 1)) / 3) ** 2;
    try {
      const t = newtonIteration(0, f);
      x0 = x0 + (x1 - x0) * t;
      y0 = y0 + (y1 - y0) * t;
    } catch (error) {
      console.log(error);
    }

    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      selectable: true,
      stroke: 'black',
      strokeDashArray: [3, 2], // strokeDashArray[a,b] =》 每隔a个像素空b个像素
    });
    canvas.add(path);

    // 绘制行星名
    const fontSize = config.fontSize;

    let color = '#28a745';
    if (planets[i].speed < 0) color = '#dc3545';
    if (planets[i].is_stationary) color = '#ffc107';

    const planetText = new fabric.Text(planets[i].name, {
      fontSize: fontSize,
      selectable: true,
      stroke: color,
      // fontFamily: config.planetFontFamily(planets[i].name),
    });
    planetText.left = x - planetText.width! / 2;
    planetText.top = y - planetText.height! / 2;

    const planetLongOnZoodiac = zodiacLong(planets[i].long);
    const planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);

    const xiuDMS = degreeToDMS(planets[i].xiu_degree);

    let message = `${planets[i].name}
${planetLongOnZoodiac.zodiac}宫：${planetLongDMSOnZoodiac.d}度${planetLongDMSOnZoodiac.m}分${planetLongDMSOnZoodiac.s}秒
${planets[i].xiu}宿：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒
${planets[i].speed_state}`;

    if (planets[i].speed < 0) message = `${message}、逆`;
    else message = `${message}、顺`;

    if (planets[i].is_stationary) message = `${message}、留`;

    const noteText = tip.newTip(message, planetText, canvas);

    planetText.on('mousedown', (e) => {
      tip.showTip(noteText, canvas);
    });

    canvas.add(planetText);
  }
}

/**
 * 画二十八宿
 * @param distanceStarLongs
 * @param canvas
 * @param tip
 * @param config
 * @param options
 */
function drawDistanceStar(
  distanceStarLongs: Array<DistanceStarLong>,
  canvas: fabric.Canvas,
  tip: TipService,
  config: QizhengConfigService,
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

  distanceStarLongs.forEach((distance_star, index) => {
    // 画分隔线
    const x0 = cx + (r0 - 6) * cos(distance_star.long - 30);
    const y0 = cy - (r0 - 6) * sin(distance_star.long - 30);

    const x1 = cx + (r1 + 4) * cos(distance_star.long - 30);
    const y1 = cy - (r1 + 4) * sin(distance_star.long - 30);

    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      selectable: true,
      stroke: '#f28b82',
      // strokeDashArray: [3, 2], // strokeDashArray[a,b] =》 每隔a个像素空b个像素
    });
    canvas.add(path);

    // 画文字
    const next_distance_star =
      distanceStarLongs[(index + 1) % distanceStarLongs.length];

    // r=r1+(r0-r1)/2=(r0+r1)/2
    // long = long0+(long1-long0)/2=(long0+long1)/2
    // const long = (next_distance_star.long + distance_star.long) / 2;
    const long = degNorm(
      degNorm(next_distance_star.long - distance_star.long) / 2 +
        distance_star.long
    );
    const x = cx + ((r0 + r1) / 2) * cos(long - 30);
    const y = cy - ((r0 + r1) / 2) * sin(long - 30);

    const planetText = new fabric.Text(distance_star.lunar_mansions, {
      fontSize: config.fontSize,
      selectable: true,
      // stroke: 'black',
      // fontFamily: config.planetFontFamily(planets[i].name),
    });
    planetText.left = x - planetText.width! / 2;
    planetText.top = y - planetText.height! / 2;

    // 距星经度
    const planetLongOnZoodiac = zodiacLong(distance_star.long);
    const planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);

    // 宿宽度
    const xiuWidth = degNorm(next_distance_star.long - distance_star.long);
    const xiuDMS = degreeToDMS(xiuWidth);

    let message = `${distance_star.lunar_mansions}
${planetLongOnZoodiac.zodiac}宫：${planetLongDMSOnZoodiac.d}度${planetLongDMSOnZoodiac.m}分${planetLongDMSOnZoodiac.s}秒
宿宽：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`;

    const noteText = tip.newTip(message, planetText, canvas);

    planetText.on('mousedown', (e) => {
      tip.showTip(noteText, canvas);
    });

    canvas.add(planetText);
  });

  // // 画宫位分隔线
  // for (let i = 0; i < 12; i++) {
  //   // 画宫位分隔线
  //   const x0 = cx + r1 * cos(30 * i);
  //   const y0 = cx - r1 * sin(30 * i);

  //   const x1 = cx + r0 * cos(30 * i);
  //   const y1 = cx - r0 * sin(30 * i);
  //   let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
  //     stroke: 'black',
  //     selectable: false,
  //     strokeDashArray: [3, 2], // strokeDashArray[a,b] =》 每隔a个像素空b个像素
  //   });
  //   canvas.add(path);
  // }
}

function drawDongWei(
  dong_wei: DongWei,
  canvas: fabric.Canvas,
  tip: TipService,
  config: QizhengConfigService,
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

  for (let i = 0; i < dong_wei.long_of_per_year.length - 1; i++) {
    // 画分隔线
    const x0 = cx + r0 * cos(dong_wei.long_of_per_year[i] - 30);
    const y0 = cy - r0 * sin(dong_wei.long_of_per_year[i] - 30);

    const x1 = cx + r1 * cos(dong_wei.long_of_per_year[i] - 30);
    const y1 = cy - r1 * sin(dong_wei.long_of_per_year[i] - 30);

    let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
      selectable: true,
      stroke: 'black',
      // strokeDashArray: [3, 2], // strokeDashArray[a,b] =》 每隔a个像素空b个像素
    });
    canvas.add(path);

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

    const planetText = new fabric.Text(`${i + 1}`, {
      fontSize: config.fontSize / 2,
      selectable: true,
      // stroke: 'black',
      // fontFamily: config.planetFontFamily(planets[i].name),
    });
    planetText.left = x - planetText.width! / 2;
    planetText.top = y - planetText.height! / 2;

    canvas.add(planetText);
  }

  // 画当前的洞微
  // 画分隔线
  const x0 = cx + r0 * cos(dong_wei.long - 30);
  const y0 = cy - r0 * sin(dong_wei.long - 30);

  const x1 = cx + r1 * cos(dong_wei.long - 30);
  const y1 = cy - r1 * sin(dong_wei.long - 30);

  let path = new fabric.Path(`M ${x0}, ${y0} L ${x1} ${y1}`, {
    selectable: true,
    stroke: 'blue',
    strokeWidth: 5,
    // strokeDashArray: [3, 2], // strokeDashArray[a,b] =》 每隔a个像素空b个像素
  });

  // 距星经度
  const planetLongOnZoodiac = zodiacLong(dong_wei.long);
  const planetLongDMSOnZoodiac = degreeToDMS(planetLongOnZoodiac.long);

  // 宿宽度
  const xiuDMS = degreeToDMS(dong_wei.xiu_degree);

  let message = `${planetLongOnZoodiac.zodiac}宫：${planetLongDMSOnZoodiac.d}度${planetLongDMSOnZoodiac.m}分${planetLongDMSOnZoodiac.s}秒
${dong_wei.xiu}宿：${xiuDMS.d}度${xiuDMS.m}分${xiuDMS.s}秒`;

  const noteText = tip.newTip(message, path, canvas);

  path.on('mousedown', (e) => {
    tip.showTip(noteText, canvas);
  });

  canvas.add(path);
}
