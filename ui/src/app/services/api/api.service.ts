import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import {
  HoroRequest,
  ProfectionRequest,
  ReturnRequest,
  TransitRequest,
} from 'src/app/type/interface/request-data';
import {
  Horoscope,
  HoroscopeCompare,
  Profection,
  ReturnHoroscop,
} from 'src/app/type/interface/respone-data';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root',
})
export class ApiService {
  private readonly url = `${environment.base_url}/api`;
  private readonly http_options = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
  };

  constructor(private http: HttpClient) {}

  /**
   *
   * @returns 获取宫位系统
   */
  public getHouses(): Observable<Array<string>> {
    return this.http.get<Array<string>>(`${this.url}/houses`);
  }

  /**
   *
   * @returns 获取本命星盘
   */
  public getNative(data: HoroRequest): Observable<Horoscope> {
    return this.http.post<Horoscope>(
      `${this.url}/horo/native`,
      data,
      this.http_options
    );
  }

  /**
   *
   * @returns 获取小限
   */
  public profection(data: ProfectionRequest): Observable<Profection> {
    return this.http.post<Profection>(
      `${this.url}/process/profection`,
      data,
      this.http_options
    );
  }

  /**
   *
   * @returns 获取行运
   */
  public transit(data: TransitRequest): Observable<HoroscopeCompare> {
    return this.http.post<HoroscopeCompare>(
      `${this.url}/process/transit`,
      data,
      this.http_options
    );
  }

  /**
   *
   * @returns 获取太阳返照盘
   */
  public solarReturn(data: ReturnRequest): Observable<ReturnHoroscop> {
    return this.http.post<ReturnHoroscop>(
      `${this.url}/process/return/solar`,
      data,
      this.http_options
    );
  }

  /**
   *
   * @returns 获取月亮返照盘
   */
  public lunarReturn(data: ReturnRequest): Observable<ReturnHoroscop> {
    return this.http.post<ReturnHoroscop>(
      `${this.url}/process/return/lunar`,
      data,
      this.http_options
    );
  }
}
