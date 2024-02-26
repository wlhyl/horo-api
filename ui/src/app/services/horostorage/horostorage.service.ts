import { Injectable } from '@angular/core';

import { HoroRequest, ProcessRequest } from '../../type/interface/request-data';
import { ProcessName } from 'src/app/type/enum/process';

@Injectable({
  providedIn: 'root',
})
export class HorostorageService {
  /**
   * 本命
   */
  public set horoData(date: HoroRequest) {
    localStorage.setItem('horo_data', JSON.stringify(date));
  }

  public get horoData(): HoroRequest {
    let j = localStorage.getItem('horo_data');
    if (j) {
      return JSON.parse(j) as HoroRequest;
    }

    let t = this.nowDate();

    return {
      date: {
        year: t.year,
        month: t.month,
        day: t.day,
        hour: t.hour,
        minute: t.minute,
        second: t.second,
        tz: t.tz,
        st: t.st,
      },
      geo_name: '北京',
      geo: {
        long: 116 + 25 / 60.0,
        lat: 39 + 54 / 60.0,
      },
      house: 'Alcabitus',
      sex: true,
      describe: '',
    };
  }

  public set processData(date: ProcessRequest) {
    localStorage.setItem('process_data', JSON.stringify(date));
  }

  public get processData(): ProcessRequest {
    let j = localStorage.getItem('process_data');
    if (j) {
      return JSON.parse(j) as ProcessRequest;
    }

    let t = this.nowDate();

    return {
      date: {
        year: t.year,
        month: t.month,
        day: t.day,
        hour: t.hour,
        minute: t.minute,
        second: t.second,
        tz: t.tz,
        st: t.st,
      },
      geo_name: '北京',
      geo: {
        long: 116 + 25 / 60.0,
        lat: 39 + 54 / 60.0,
      },
      process_name: ProcessName.Profection,
      isSolarReturn: false,
    };
  }

  // //小限
  // public set profections_data(date :ProfectionData){
  //   localStorage.setItem("profections_data", JSON.stringify(date))
  // }

  // public get profections_data() :ProfectionsData{
  //   let j = localStorage.getItem("profections_data")
  //   if(j){
  //     return JSON.parse(j) as ProfectionsData
  //   }

  //   let t  = this.nowDate()

  //   return {
  //     year: t.year,
  //     month: t.month,
  //     day: t.day,
  //     hour: t.hour,
  //     minute: t.minute,
  //     second: t.second,
  //   }
  // }

  // //行运
  // public set transit_data(date :TransitData){
  //   localStorage.setItem("transit_data", JSON.stringify(date))
  // }

  // public get transit_data() :TransitData{
  //   let j = localStorage.getItem("transit_data")
  //   if(j){
  //     return JSON.parse(j) as TransitData
  //   }

  //   let t  = this.nowDate()

  //   return {
  //     year: t.year,
  //     month: t.month,
  //     day: t.day,
  //     hour: t.hour,
  //     minute: t.minute,
  //     second: t.second,
  //   }
  // }

  // // 日返
  // public set solar_return_data(date :SolarReturnData){
  //   localStorage.setItem("solar_return_data", JSON.stringify(date))
  // }

  // public get solar_return_data() :SolarReturnData{
  //   let j = localStorage.getItem("solar_return_data")
  //   if(j){
  //     return JSON.parse(j) as SolarReturnData
  //   }

  //   let t  = this.nowDate()

  //   return {
  //     year: t.year,
  //     month: t.month,
  //     day: t.day,
  //     hour: t.hour,
  //     minute: t.minute,
  //     second: t.second,
  //     tz: t.tz,
  //     sT: t.sT,
  //     geoLocalName: "北京",
  //     geoLongD: 116,
  //     geoLongM: 25,
  //     geoLongS: 0,
  //     geoEW: "E",
  //     geoLatD: 39,
  //     geoLatM: 54,
  //     geoLatS: 0,
  //     geoNS: "N",
  //     house: 'B'
  //   }
  // }

  // // 月返
  // public set lunar_return_data(date :LunarReturnData){
  //   localStorage.setItem("lunar_return_data", JSON.stringify(date))
  // }

  // public get lunar_return_data() :LunarReturnData{
  //   let j = localStorage.getItem("lunar_return_data")
  //   if(j){
  //     return JSON.parse(j) as LunarReturnData
  //   }

  //   let t  = this.nowDate()

  //   return {
  //     year: t.year,
  //     month: t.month,
  //     day: t.day,
  //     hour: t.hour,
  //     minute: t.minute,
  //     second: t.second,
  //     tz: t.tz,
  //     sT: t.sT,
  //     geo_local_name: "北京",
  //     geo_long_d: 116,
  //     geo_long_m: 25,
  //     geo_long_s: 0,
  //     geo_ew: "E",
  //     geo_lat_d: 39,
  //     geo_lat_m: 54,
  //     geo_lat_s: 0,
  //     geo_ns: "N",
  //     house: 'B',
  //     solar: false,
  //   }
  // }

  constructor() {}

  private nowDate(): date {
    let t = new Date();
    let year = t.getFullYear();
    let month = t.getMonth() + 1;
    let day = t.getDate();
    let hour = t.getHours();
    let minute = t.getMinutes();
    let second = t.getSeconds();

    let st = false;

    // 判断夏令时
    let d1 = new Date(year, 1, 1);
    let tz = d1.getTimezoneOffset() / -60;
    // let d2 = new Date(this.horo.year,7,1);
    if (t.getTimezoneOffset() != d1.getTimezoneOffset()) {
      st = true;
      tz -= 1;
    }

    return {
      year,
      month,
      day,
      hour,
      minute,
      second,
      tz,
      st,
    };
  }
}

interface date {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  tz: number;
  st: boolean;
}
