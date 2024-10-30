import {HttpClient} from "@angular/common/http";
import {Injectable} from "@angular/core";
import {Observable} from "rxjs";
import {
  HoroRequest,
  ProfectionRequest,
  ReturnRequest,
  CompareRequest,
  FirdariaRequest,
  QiZhengRequst,
} from "src/app/type/interface/request-data";
import {
  FirdariaPeriod,
  Horoscope,
  HoroscopeCompare,
  Profection,
  ReturnHoroscop,
} from "src/app/type/interface/respone-data";
import {Horoscope as QiZhengHoroscope} from "src/app/type/interface/response-qizheng";
import {environment} from "src/environments/environment";

@Injectable({
  providedIn: "root",
})
export class ApiService {
  private readonly url = `${environment.base_url}/api`;

  constructor(private http: HttpClient) {
  }

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
    return this.http.post<Horoscope>(`${this.url}/horo/native`, data);
  }

  /**
   *
   * @returns 获取小限
   */
  public profection(data: ProfectionRequest): Observable<Profection> {
    return this.http.post<Profection>(`${this.url}/process/profection`, data);
  }

  /**
   *
   * @returns 获取法达
   */
  public firdaria(data: FirdariaRequest): Observable<Array<FirdariaPeriod>> {
    return this.http.post<Array<FirdariaPeriod>>(`${this.url}/process/firdaria`, data);
  }

  /**
   *
   * @returns 获取比较盘
   */
  public compare(data: CompareRequest): Observable<HoroscopeCompare> {
    return this.http.post<HoroscopeCompare>(`${this.url}/process/compare`, data);
  }

  /**
   *
   * @returns 获取太阳返照盘
   */
  public solarReturn(data: ReturnRequest): Observable<ReturnHoroscop> {
    return this.http.post<ReturnHoroscop>(`${this.url}/process/return/solar`, data);
  }

  /**
   *
   * @returns 获取月亮返照盘
   */
  public lunarReturn(data: ReturnRequest): Observable<ReturnHoroscop> {
    return this.http.post<ReturnHoroscop>(`${this.url}/process/return/lunar`, data);
  }

  /**
   *
   * @returns 获取七政
   */
  public qizheng(data: QiZhengRequst): Observable<QiZhengHoroscope> {
    return this.http.post<QiZhengHoroscope>(`${this.url}/qizheng/horo`, data);
  }
}
