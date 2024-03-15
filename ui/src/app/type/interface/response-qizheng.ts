import {
  HouseName,
  LunarMansionsName,
  PlanetName,
  PlanetSpeedState,
} from '../enum/qizheng';
import { GeoPosition, HoroDateTime } from './respone-data';

export interface Planet {
  name: PlanetName;
  // 行星的黄经
  long: number;
  // 行星在黄道上每日的移动速度
  speed: number;

  // 行星在黄道上的宿
  xiu: LunarMansionsName;

  // 行星在黄道上的入宿度
  xiu_degree: number;
  // 行星速度状态：快、平均、慢
  speed_state: PlanetSpeedState;
  // 停滞，行星移动速度小于1度，是停滞，只有，水、金、火、木、土，有停滞
  is_stationary: boolean;
}

/**
 * 二十八宿的黄道经度
 */
export interface DistanceStarLong {
  // 二十八宿的名称
  lunar_mansions: LunarMansionsName;
  // 二十八宿距星的黄道经度
  long: number;
}

export interface ASCHouse {
  // 上升点度数，没换算成上升点所在星座的度数
  asc_long: number;
  // 命度所在宿名
  xiu: LunarMansionsName;
  // 入宿度数
  xiu_degree: number;
}

export interface House {
  // 宫位的名称
  name: HouseName;

  // 宫头的黄道经度
  long: number;

  // 宫头所在宿
  xiu: LunarMansionsName;

  // 宫头入宿度
  xiu_degree: number;
}

export interface DongWei {
  // 洞微大限每一年的黄道经度，从0岁起至洞微大限总年数，洞微大限总年数略去小数部分，起算点为每年的公历生日
  long_of_per_year: Array<number>;

  // 当前推运时间的洞微大限黄道经度
  long: number;

  // 当前推运时间的洞微大限黄道经度所在宿名
  xiu: LunarMansionsName;
  // 当前推运时间的洞微大限道经度的入宿度数
  xiu_degree: number;
}

export interface Horoscope {
  // 出生时间
  native_date: HoroDateTime;
  // 推运时间
  process_date: HoroDateTime;
  // 出生地大地经纬度
  geo: GeoPosition;
  // 十一颗行星
  // 本命行星
  native_planets: Array<Planet>;
  // 流年行星
  process_planets: Array<Planet>;
  // 距星的黄道经度和名称
  distance_star_long: Array<DistanceStarLong>;
  // 命宫
  asc_house: ASCHouse;
  // 宫位
  houses: Array<House>;

  ///  出生时刻的农历
  native_lunar_calendar: LunarCalendar;
  /// 推运时刻的农历
  process_lunar_calendar: LunarCalendar;

  // 洞微大限
  dong_wei: DongWei;

  //    @field:Schema(description = "本命纳间")
  //    val naYin = getNaYinData(nativeLunarCalendar.yearGanZhi)

  //    @field:Schema(description = "本命变曜")
  //    val nativeVirtualStars =  getVirtualStars(nativeTime, geo, ephePath)

  //    @field:Schema(description = "流年变曜")
  //    val processVirtualStars =  getVirtualStars(processTime, geo, ephePath)

  //    @field:Schema(description = "本命神煞")
  //    val nativeShenShas = getShenShas(nativeTime, geo, ephePath)

  //    @field:Schema(description = "流年变曜")
  //    val processShenShas = getShenShas(processTime, geo, ephePath)
}

export interface LunarCalendar {
  // 闰年:true
  is_lean_year: boolean;

  // 农历年，干支表示
  lunar_year: string; // GanZhi,

  // 农历月，以正月、二月、......、十月、冬月、腊月表示
  lunar_month: string;

  // 农历日，以初一、初二、……、二十九、三十表示
  lunar_day: string;

  // 农历年干支，按节气换年
  lunar_year_gan_zhi: string; // GanZhi,

  // 农历月干支，按节气换月
  lunar_month_gan_zhi: string; // GanZhi,

  // 日干支
  lunar_day_gan_zhi: string; // GanZhi,

  // 时干支
  time_gan_zhi: string; // GanZhi,

  // 节
  solar_term_first: SolarTerm;

  // 中气
  solar_term_second: SolarTerm;
}

// 节气
export interface SolarTerm {
  name: string;
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
}
