import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import {
  HoroData,
  ProfectionData,
  TransitData,
} from 'src/app/type/interface/request-data';
import {
  Horoscope,
  HoroscopeCompare,
  Profection,
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
    return this.http.get<Array<string>>(`${this.url}/horo/houses`);
  }

  /**
   *
   * @returns 获取本命星盘
   */
  public getNative(data: HoroData): Observable<Horoscope> {
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
  public profection(data: ProfectionData): Observable<Profection> {
    return this.http.post<Profection>(
      `${this.url}/horo/profection`,
      data,
      this.http_options
    );
  }

  /**
   *
   * @returns 获取行运
   */
  public transit(data: TransitData): Observable<HoroscopeCompare> {
    return this.http.post<HoroscopeCompare>(
      `${this.url}/horo/transit`,
      data,
      this.http_options
    );
  }
}
