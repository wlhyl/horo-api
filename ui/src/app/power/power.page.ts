import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Horoconfig } from '../services/config/horo-config.service';
import { Zodiac } from '../type/enum/zodiac';
import {
  detriment,
  egyptianTerm,
  exaltation,
  face,
  fall,
  ptolemyTerm,
  rulership,
  tripilicity,
  tripilicityOfLily,
} from '../utils/image/zodiac';

@Component({
  selector: 'app-power',
  templateUrl: './power.page.html',
  styleUrls: ['./power.page.scss'],
})
export class PowerPage implements OnInit {
  title = '行星力量表';

  rulershipFn = rulership;
  exaltationFn = exaltation;
  tripilicityFn = tripilicity;
  tripilicityOfLilyFn = tripilicityOfLily;
  faceFn = face;
  detrimentFn = detriment;
  fallFn = fall;
  ptolemyTermFn = ptolemyTerm;
  egyptianTermFn = egyptianTerm;

  zodiacs = [
    Zodiac.Aries,
    Zodiac.Taurus,
    Zodiac.Gemini,
    Zodiac.Cancer,
    Zodiac.Leo,
    Zodiac.Virgo,
    Zodiac.Libra,
    Zodiac.Scorpio,
    Zodiac.Sagittarius,
    Zodiac.Capricorn,
    Zodiac.Aquarius,
    Zodiac.Pisces,
  ];
  constructor(public config: Horoconfig, private titleService: Title) {}

  // rulerships = [
  //   PlanetName.Mars,
  //   PlanetName.Venus,
  //   PlanetName.Mercury,
  //   PlanetName.Moon,
  //   PlanetName.Sun,
  //   PlanetName.Mercury,
  //   PlanetName.Venus,
  //   PlanetName.Mars,
  //   PlanetName.Jupiter,
  //   PlanetName.Saturn,
  //   PlanetName.Saturn,
  //   PlanetName.Jupiter,
  // ];
  //     exaltation: PlanetName.Sun,
  //     tripilicity: [PlanetName.Sun, PlanetName.Jupiter],
  //    ptolemy_term: [{PlanetName. Jupiter : 6,},],
  //     face: [PlanetName.Mars, PlanetName.Sun, PlanetName.Venus],
  //     detriment: PlanetName.Venus,
  //     fall: PlanetName.Saturn,
  //   },
  // ];
  ngOnInit() {
    this.titleService.setTitle(this.title);
  }
}
