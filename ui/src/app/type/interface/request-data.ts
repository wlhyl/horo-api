import { ProcessName } from '../enum/process';

/**
 * 本命星盘请求数据
 */
export interface HoroData {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number;
  st: boolean;
  geo_name: string;
  geo_long: number;
  geo_lat: number;
  house: string;
  describe: string;
  sex: boolean;
}

// /**
//  * 推运星盘请求数据
//  */
// export interface ProfectionData {
//   year: number;
//   month: number;
//   day: number;
//   hour: number;
//   minute: number;
//   second: number;
// }

/**
 * 推运星盘请求数据
 */
export interface ProcessData {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  // tz: number;
  // st: boolean;
  geo_name: string;
  geo_long: number;
  geo_lat: number;
  // house: string;
  // describe: string;
  // sex: boolean;
  process_name: ProcessName;
}

export interface ProfectionData {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  // 出生地时区，东区为正数，西区为负数
  tz: number;
  // 出生时的夏令时，有夏令时：true，无夏令时： false
  st: boolean;

  // 推运年，最小值1900
  process_year: number;
  // 推运月
  process_month: number;

  // 推运日
  process_day: number;
  // 推运时
  process_hour: number;
  // 推运分
  process_minute: number;
  // 推运秒
  process_second: number;
}
