import { Injectable } from '@angular/core';
import { HttpHeaders } from '@angular/common/http';

import { PlanetName } from '../../type/enum/planet';
import { Zodiac } from '../../type/enum/zodiac';

@Injectable({
  providedIn: 'root',
})
export class Horoconfig {
  // public readonly baseUrl :string = environment.base_url
  // public readonly qizhengBaseUrl :string = environment.qizheng_base_url

  // public readonly houseSystemUrl :string = this.baseUrl + '/api/horo/supporthouses'
  // public readonly horoHoroscopeUrl :string = this.baseUrl + '/api/horo/horohscope'
  // // public readonly horo_planets_url :string = this.baseUrl + '/horo/planets'
  // // public readonly horo_asmc_url :string = this.base_url + '/horo/asmc'
  // public readonly horoProfectionsUrl :string = this.baseUrl + '/horo/profections'
  // public readonly horoFirdariaUrl :string = this.baseUrl + '/horo/firdaria'
  // public readonly horoTransitUrl :string = this.baseUrl + '/horo/transit'
  // public readonly horoSolarReturnUrl :string = this.baseUrl + '/horo/solarreturn'
  // public readonly horoLunarReturnUrl :string = this.baseUrl + '/horo/lunarreturn'

  // public readonly qizhengHorohscopeUrl :string = this.qizhengBaseUrl + '/qizheng/horohscope'

  /**
   * 西占行星列表
   */
  // 必需依此顺序排序，因为绘制相位图时，会依此顺序
  public readonly horoPlanets = [
    PlanetName.Sun,
    PlanetName.Moon,
    PlanetName.Mercury,
    PlanetName.Venus,
    PlanetName.Mars,
    PlanetName.Jupiter,
    PlanetName.Saturn,
    PlanetName.NorthNode,
    PlanetName.SouthNode,
    PlanetName.ASC,
    PlanetName.MC,
    PlanetName.DSC,
    PlanetName.IC,
  ];

  /**
   * 占星字体
   */
  public readonly astrologyFont = 'HamburgSymbols';
  /**
   * 普通文本字体
   */
  public readonly textFont = 'Verdana';

  /**
   *
   * @param planet 行星的id
   * @returns 返回行星的可输出字符串
   */
  public planetFontString(planet: PlanetName): string {
    let planetString = new Map([
      [PlanetName.Sun, 'Q'],
      [PlanetName.Moon, 'W'],
      [PlanetName.Mercury, 'E'],
      [PlanetName.Venus, 'R'],
      [PlanetName.Mars, 'T'],
      [PlanetName.Jupiter, 'Y'],
      [PlanetName.Saturn, 'U'],
      [PlanetName.NorthNode, '{'],
      [PlanetName.SouthNode, '}'],
      [PlanetName.ASC, 'ASC'],
      [PlanetName.MC, 'MC'],
      [PlanetName.DSC, 'DSC'],
      [PlanetName.IC, 'IC'],
    ]);
    if (planetString.has(planet)) {
      return planetString.get(planet) as string;
    } else {
      return planet.toString();
    }
  }

  /**
   *
   * @param aspect 相位值
   * @returns 相位的可输出字符串
   */
  public aspectFontString(aspect: number): string {
    switch (aspect) {
      case 0:
        return 'q';
      case 60:
        return 't';
      case 90:
        return 'r';
      case 120:
        return 'e';
      case 180:
        return 'w';
      default:
        return '';
    }
  }

  /**
   *
   * @param planet 行星的id
   * @returns  返回输星的输出字体
   */
  public planetFontFamily(planet: PlanetName): string {
    if (
      [PlanetName.ASC, PlanetName.MC, PlanetName.DSC, PlanetName.IC].includes(
        planet
      )
    ) {
      return this.textFont;
    } else {
      return this.astrologyFont;
    }
  }

  /**
   *
   * @returns 相位字体
   */
  public aspectFontFamily(): string {
    return this.astrologyFont;
  }

  /**
   *
   * @param zodiac 12星座的id
   * @returns 星座的可打印字符
   */
  public zodiacFontString(zodiac: Zodiac): string {
    let zodiacString = new Map([
      [Zodiac.Aries, 'a'],
      [Zodiac.Taurus, 's'],
      [Zodiac.Gemini, 'd'],
      [Zodiac.Cancer, 'f'],
      [Zodiac.Leo, 'g'],
      [Zodiac.Virgo, 'h'],
      [Zodiac.Libra, 'j'],
      [Zodiac.Scorpio, 'k'],
      [Zodiac.Sagittarius, 'l'],
      [Zodiac.Capricorn, 'z'],
      [Zodiac.Aquarius, 'x'],
      [Zodiac.Pisces, 'c'],
    ]);
    if (zodiacString.has(zodiac)) return zodiacString.get(zodiac) as string;
    return zodiac.toString();
  }

  /**
   *
   * @returns 星座的可打印字体
   */
  public zodiacFontFamily(): string {
    return this.astrologyFont;
  }

  // 初始宽、高，绘制完成后会根据屏幕大小缩放
  public apsectImage = { width: 700, heigth: 700 };
  public HoroscoImage = { width: 700, heigth: 700 }; // , fontSize: 20, col: 14, row: 14}

  //   public readonly ruler = [
  //     Planet.SE_MARS,
  //     Planet.SE_VENUS,
  //     Planet.SE_MERCURY,
  //     Planet.SE_MOON,
  //     Planet.SE_SUN,
  //     Planet.SE_MERCURY,
  //     Planet.SE_VENUS,
  //     Planet.SE_MARS,
  //     Planet.SE_JUPITER,
  //     Planet.SE_SATURN,
  //     Planet.SE_SATURN,
  //     Planet.SE_JUPITER,
  //   ]

  //   public readonly exaltation  = [
  //     Planet.SE_SUN,
  //     Planet.SE_MOON,
  //     -10000, // -10000，表示无
  //     Planet.SE_JUPITER,
  //     -10000,
  //     Planet.SE_MERCURY,
  //     Planet.SE_SATURN,
  //     -10000,
  //     -10000,
  //     Planet.SE_MARS,
  //     -10000,
  //     Planet.SE_VENUS
  //   ]

  //   public readonly egypian_terms :any = [
  //     [
  //       [Planet.SE_JUPITER, "0 - 6"],
  //       [Planet.SE_VENUS,"6 - 12"],
  //       [Planet.SE_MERCURY, "12 - 20"],
  //       [Planet.SE_MARS,"20 - 25"],
  //       [Planet.SE_SATURN, "25 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER,"0 - 8"],
  //       [Planet.SE_VENUS, "8 - 14"],
  //       [Planet.SE_MERCURY, "14 - 22"],
  //       [Planet.SE_MARS, "22 - 27"],
  //       [Planet.SE_SATURN, "27 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 6"],
  //       [Planet.SE_VENUS, "6 - 12"],
  //       [Planet.SE_MERCURY, "12 - 17"],
  //       [Planet.SE_MARS, "17 - 24"],
  //       [Planet.SE_SATURN, "24 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 7"],
  //       [Planet.SE_VENUS, "7 - 13"],
  //       [Planet.SE_MERCURY, "13 - 19"],
  //       [Planet.SE_MARS, "19 - 26"],
  //       [Planet.SE_SATURN, "26 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 6"],
  //       [Planet.SE_VENUS, "6 - 11"],
  //       [Planet.SE_MERCURY, "11 - 18"],
  //       [Planet.SE_MARS, "18 - 24"],
  //       [Planet.SE_SATURN, "24 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 7"],
  //       [Planet.SE_VENUS, "7 - 17"],
  //       [Planet.SE_MERCURY, "17 - 21"],
  //       [Planet.SE_MARS, "21 - 28"],
  //       [Planet.SE_SATURN, "28 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 6"],
  //       [Planet.SE_VENUS, "6 - 14"],
  //       [Planet.SE_MERCURY, "14 - 21"],
  //       [Planet.SE_MARS, "21 - 28"],
  //       [Planet.SE_SATURN, "28 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 7"],
  //       [Planet.SE_VENUS, "7 - 11"],
  //       [Planet.SE_MERCURY, "11 - 19"],
  //       [Planet.SE_MARS, "19 - 24"],
  //       [Planet.SE_SATURN, "24 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 12"],
  //       [Planet.SE_VENUS, "12 - 17"],
  //       [Planet.SE_MERCURY, "17 - 21"],
  //       [Planet.SE_MARS, "21 - 26"],
  //       [Planet.SE_SATURN, "26 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 7"],
  //       [Planet.SE_VENUS, "7 - 14"],
  //       [Planet.SE_MERCURY, "17 - 22"],
  //       [Planet.SE_MARS, "22 - 26"],
  //       [Planet.SE_SATURN, "26 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 7"],
  //       [Planet.SE_VENUS, "7 - 13"],
  //       [Planet.SE_MERCURY, "13 - 20"],
  //       [Planet.SE_MARS, "20 - 25"],
  //       [Planet.SE_SATURN, "25 - 30"],
  //     ],
  //     [
  //       [Planet.SE_JUPITER, "0 - 12"],
  //       [Planet.SE_VENUS, "12 - 16"],
  //       [Planet.SE_MERCURY, "16 - 19"],
  //       [Planet.SE_MARS, "19 - 28"],
  //       [Planet.SE_SATURN, "28 - 30"],
  //     ]
  // ]
  //   public readonly faces = [
  //       [Planet.SE_MARS, Planet.SE_SUN, Planet.SE_VENUS],
  //       [Planet.SE_MERCURY, Planet.SE_MOON, Planet.SE_SATURN],
  //       [Planet.SE_JUPITER, Planet.SE_MARS, Planet.SE_SUN],
  //       [Planet.SE_VENUS, Planet.SE_MERCURY, Planet.SE_MOON],
  //       [Planet.SE_SATURN, Planet.SE_JUPITER, Planet.SE_MARS],
  //       [Planet.SE_SUN, Planet.SE_VENUS, Planet.SE_MERCURY],
  //       [Planet.SE_MOON, Planet.SE_SATURN, Planet.SE_JUPITER],
  //       [Planet.SE_MARS, Planet.SE_SUN, Planet.SE_VENUS],
  //       [Planet.SE_MERCURY, Planet.SE_MOON, Planet.SE_SATURN],
  //       [Planet.SE_JUPITER, Planet.SE_MARS, Planet.SE_SUN],
  //       [Planet.SE_VENUS, Planet.SE_MERCURY, Planet.SE_MOON],
  //       [Planet.SE_SATURN, Planet.SE_JUPITER, Planet.SE_MARS],
  //   ]
  public readonly httpOptions = {
    headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
  };

  constructor() {}

  // getHouseSystemUrl() :string{
  //   return this.house_system_url
  // }
}
