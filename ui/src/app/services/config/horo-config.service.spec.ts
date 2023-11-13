import { TestBed } from '@angular/core/testing';

import { Horoconfig} from './horo-config.service';
import { Planet } from '../../type/enum/planet'
import { Zodiac } from '../../type/enum/zodiac'

describe('HoroconfigService', () => {
  let service: Horoconfig;

  beforeEach(() => {
    TestBed.configureTestingModule({providers: [Horoconfig]});
    service = TestBed.inject(Horoconfig);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('api urls is defined', () =>{
    let urls = [
      service.baseUrl, service.qizhengBaseUrl, service.houseSystemUrl, service.horoHoroscopeUrl,
      service.horoProfectionsUrl, service.horoFirdariaUrl, service.horoTransitUrl, service.horoSolarReturnUrl,
      service.horoLunarReturnUrl, service.qizhengHorohscopeUrl
    ]
    for(let url of urls){
      expect(url).toBeDefined()
    }
  })

  it('西占行星列表', () => {
    let horoPlanets = [
      Planet.SE_SUN,
      Planet.SE_MOON,
      Planet.SE_MERCURY,
      Planet.SE_VENUS,
      Planet.SE_MARS,
      Planet.SE_JUPITER,
      Planet.SE_SATURN,
      Planet.HORO_NORTH_NODE,
      Planet.HORO_SOUTH_NODE,    
      Planet.HORO_ASC,
      Planet.HORO_MC,
      Planet.HORO_DSC,
      Planet.HORO_IC,
    ]
    expect(service.horoPlanets).toEqual(horoPlanets)
  })

  it('占星字体', () => {
    expect(service.astrologyFont).toBeDefined()
  })

  it('普通文本字体', () => {
    expect(service.textFont).toBeDefined()
  })

  it('行星字体符号', () =>{
    let planetStrings = [
      {planet :Planet.SE_SUN, string: 'Q'},
      {planet :Planet.SE_MOON, string: 'W'},
      {planet: Planet.SE_MERCURY, string: 'E'},
      {planet: Planet.SE_VENUS,string: 'R'},
      {planet: Planet.SE_MARS, string: 'T'},
      {planet: Planet.SE_JUPITER, string: 'Y'},
      {planet: Planet.SE_SATURN, string: 'U'},
      {planet: Planet.SE_TRUE_NODE, string: '{'},
      {planet: -Planet.SE_TRUE_NODE, string: '}' },
      {planet: Planet.SE_MEAN_NODE, string: '{' },
      {planet: -Planet.SE_MEAN_NODE, string: '}' },
      {planet: Planet.HORO_ASC, string: 'ASC' },
      {planet: Planet.HORO_MC,string: 'MC'},
      {planet: Planet.HORO_DSC, string: 'DSC'},
      {planet: Planet.HORO_IC, string: 'IC'}
    ]
    for(let planetString of planetStrings){
      expect(service.planetFontString(planetString.planet)).toEqual(planetString.string)
    }
  })

  it('相位字体字符', () => {
    let aspects = [
      {aspect: 0, string: "q"},
      {aspect: 60, string: 't'},
      {aspect: 90, string: 'r'},
      {aspect: 120, string: 'e'},
      {aspect: 180, string: 'w'},
    ]
    for(let aspect of aspects){
      expect(service.aspectFontString(aspect.aspect)).toEqual(aspect.string)
    }
  })

  it('行星字体', () => {
    let angularHouseCups = [Planet.HORO_ASC, Planet.HORO_MC, Planet.HORO_DSC, Planet.HORO_IC]
    for(let cups of angularHouseCups){
      expect(service.planetFontFamily(cups)).toEqual(service.textFont)
    }

    let planets = [
      Planet.SE_SUN,
      Planet.SE_MOON,
      Planet.SE_MERCURY,
      Planet.SE_VENUS,
      Planet.SE_MARS,
      Planet.SE_JUPITER,
      Planet.SE_SATURN,
      Planet.HORO_NORTH_NODE,
      Planet.HORO_SOUTH_NODE,
    ]

    for(let planet of planets){
      expect(service.planetFontFamily(planet)).toEqual(service.astrologyFont)
    }
  })

  it('相位字体', () => {
    expect(service.aspectFontFamily()).toEqual(service.astrologyFont)
  })

  it('星座字体字符', () => {
    let zodiacStrings = [
      {zodiac: Zodiac.Aries, string: "a"},
      {zodiac: Zodiac.Taurus, string: "s"},
      {zodiac: Zodiac.Gemini, string: "d"},
      {zodiac: Zodiac.Cancer, string: "f"},
      {zodiac: Zodiac.Leo, string: "g"},
      {zodiac: Zodiac.Virgo, string: "h"},
      {zodiac: Zodiac.Libra, string: "j"},
      {zodiac: Zodiac.Scorpio, string: "k"},
      {zodiac: Zodiac.Sagittarius, string: "l"},
      {zodiac: Zodiac.Capricorn, string: "z"},
      {zodiac: Zodiac.Aquarius, string: "x"},
      {zodiac: Zodiac.Pisces, string: "c"}
    ]
    for(let zodiacString of zodiacStrings){
      expect(service.zodiacFontString(zodiacString.zodiac)).toEqual(zodiacString.string)
    }
  })

  it('星座字体', () => {
    expect(service.zodiacFontFamily()).toEqual(service.astrologyFont)
  })
})



