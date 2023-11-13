import { PlanetName, PlanetSpeedState } from '../enum/planet';

export interface HoroDateTime {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number; //时区
}

export interface GeoPosition {
  long: number;
  lat: number;
}

/**
 * 本命星盘行星
 */
export interface Planet {
  // id,     七颗行星，北交，莉莉丝按瑞士星历表的行星编号。ASC: -1, MC: -2, DES: -3, IC: -4。南交点：北交点*-1
  name: PlanetName;

  // 行星的黄经
  long: number;
  //行星的黄纬
  lat: number;
  // 行星在黄道上每日的移动速度
  speed: number;
  // 行星的赤经
  ra: number;
  // 行星的赤纬
  dec: number;
  // 行星的容许度
  orb: number;
  //   星速度状态：快、平均、慢
  speed_state: PlanetSpeedState;
}

/**
 * 相位
 */
export interface Aspect {
  aspect_value: number;
  apply: boolean;
  d: number;
  p0: PlanetName;
  p1: PlanetName;
}

/**
 * 本命星盘的返回数据
 */
export interface Horosco {
  date: HoroDateTime;
  geo: GeoPosition;
  house_name: string;

  houses_cups: Array<number>;
  asc: Planet;
  mc: Planet;
  dsc: Planet;
  ic: Planet;
  planets: Array<Planet>;
  is_diurnal: boolean;
  planetary_day: PlanetName;
  planetary_hours: PlanetName;
  aspects: Array<Aspect>;
}

export interface Profection {
  // 年小限所在宫位
  year_house: number;
  // 月小限所在宫位
  month_house: number;
  // 日小限所在宫位
  day_house: number;
  // 每宫对应的日小限开始时间
  date_per_house: Array<HoroDateTime>;
}
