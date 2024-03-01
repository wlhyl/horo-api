import { ProcessName } from '../enum/process';

export interface DateRequest {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number;
  st: boolean;
}

export interface GeoRequest {
  long: number;
  lat: number;
}

/**
 * 本命星盘请求数据
 */
export interface HoroRequest {
  // 出生时间
  date: DateRequest;
  geo_name: string;
  geo: GeoRequest;
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
export interface ProcessRequest {
  date: DateRequest;
  geo_name: string;
  geo: GeoRequest;
  // house: string;
  // describe: string;
  // sex: boolean;
  process_name: ProcessName;
  isSolarReturn: boolean;
}

export interface ProfectionRequest {
  native_date: DateRequest;
  process_date: DateRequest;
}

// 行运请求数据
export interface CompareRequest {
  native_date: DateRequest;
  geo: GeoRequest;
  // 位系统，Alcabitus：阿卡比特
  house: string;
  process_date: DateRequest;
}

// 返照盘请求数据
export interface ReturnRequest {
  /// 出生时间
  native_date: DateRequest;
  /// 推运时间
  process_date: DateRequest;
  /// 居住地大地经纬度
  geo: GeoRequest;
  /// 宫位系统，Alcabitus：阿卡比特
  house: string;
}

/**
 * 法达
 */
export interface FirdariaRequest {
  // 出生时间
  native_date: DateRequest;

  // 出生地大地经纬度
  geo: GeoRequest;
}
