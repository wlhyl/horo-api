import { PlanetName } from 'src/app/type/enum/planet';
import { Zodiac } from 'src/app/type/enum/zodiac';

export function rulership(z: Zodiac): PlanetName {
  switch (z) {
    case Zodiac.Aries:
      return PlanetName.Mars;
    case Zodiac.Taurus:
      return PlanetName.Venus;
    case Zodiac.Gemini:
      return PlanetName.Mercury;
    case Zodiac.Cancer:
      return PlanetName.Moon;
    case Zodiac.Leo:
      return PlanetName.Sun;
    case Zodiac.Virgo:
      return PlanetName.Mercury;
    case Zodiac.Libra:
      return PlanetName.Venus;
    case Zodiac.Scorpio:
      return PlanetName.Mars;
    case Zodiac.Sagittarius:
      return PlanetName.Jupiter;
    case Zodiac.Capricorn:
      return PlanetName.Saturn;
    case Zodiac.Aquarius:
      return PlanetName.Saturn;
    case Zodiac.Pisces:
      return PlanetName.Jupiter;
  }
}

export function exaltation(z: Zodiac): PlanetName | null {
  switch (z) {
    case Zodiac.Aries:
      return PlanetName.Sun;
    case Zodiac.Taurus:
      return PlanetName.Moon;
    case Zodiac.Gemini:
      return PlanetName.NorthNode;
    case Zodiac.Cancer:
      return PlanetName.Jupiter;
    case Zodiac.Leo:
      return null;
    case Zodiac.Virgo:
      return PlanetName.Mercury;
    case Zodiac.Libra:
      return PlanetName.Saturn;
    case Zodiac.Scorpio:
      return null;
    case Zodiac.Sagittarius:
      return PlanetName.SouthNode;
    case Zodiac.Capricorn:
      return PlanetName.Mars;
    case Zodiac.Aquarius:
      return null;
    case Zodiac.Pisces:
      return PlanetName.Venus;
  }
}

export function tripilicity(z: Zodiac): Array<PlanetName> {
  switch (z) {
    case Zodiac.Aries:
    case Zodiac.Leo:
    case Zodiac.Sagittarius:
      return [PlanetName.Sun, PlanetName.Jupiter, PlanetName.Saturn];
    case Zodiac.Capricorn:
    case Zodiac.Taurus:
    case Zodiac.Virgo:
      return [PlanetName.Venus, PlanetName.Moon, PlanetName.Mars];
    case Zodiac.Libra:
    case Zodiac.Aquarius:
    case Zodiac.Gemini:
      return [PlanetName.Saturn, PlanetName.Mercury, PlanetName.Jupiter];
    case Zodiac.Cancer:
    case Zodiac.Scorpio:
    case Zodiac.Pisces:
      return [PlanetName.Venus, PlanetName.Mars, PlanetName.Moon];
  }
}


export function tripilicityOfLily(z: Zodiac): Array<PlanetName> {
  switch (z) {
    case Zodiac.Aries:
    case Zodiac.Leo:
    case Zodiac.Sagittarius:
      return [PlanetName.Sun, PlanetName.Jupiter];
    case Zodiac.Capricorn:
    case Zodiac.Taurus:
    case Zodiac.Virgo:
      return [PlanetName.Venus, PlanetName.Moon];
    case Zodiac.Libra:
    case Zodiac.Aquarius:
    case Zodiac.Gemini:
      return [PlanetName.Saturn, PlanetName.Mercury];
    case Zodiac.Cancer:
    case Zodiac.Scorpio:
    case Zodiac.Pisces:
      return [PlanetName.Mars, PlanetName.Mars];
  }
}

export function face(z: Zodiac): Array<PlanetName> {
  switch (z) {
    case Zodiac.Aries:
      return [PlanetName.Mars, PlanetName.Sun, PlanetName.Venus];
    case Zodiac.Taurus:
      return [PlanetName.Mercury, PlanetName.Moon, PlanetName.Saturn];
    case Zodiac.Gemini:
      return [PlanetName.Jupiter, PlanetName.Mars, PlanetName.Sun];
    case Zodiac.Cancer:
      return [PlanetName.Venus, PlanetName.Mercury, PlanetName.Moon];
    case Zodiac.Leo:
      return [PlanetName.Saturn, PlanetName.Jupiter, PlanetName.Mars];
    case Zodiac.Virgo:
      return [PlanetName.Sun, PlanetName.Venus, PlanetName.Mercury];
    case Zodiac.Libra:
      return [PlanetName.Moon, PlanetName.Saturn, PlanetName.Jupiter];
    case Zodiac.Scorpio:
      return [PlanetName.Mars, PlanetName.Sun, PlanetName.Venus];
    case Zodiac.Sagittarius:
      return [PlanetName.Mercury, PlanetName.Moon, PlanetName.Saturn];
    case Zodiac.Capricorn:
      return [PlanetName.Jupiter, PlanetName.Mars, PlanetName.Sun];
    case Zodiac.Aquarius:
      return [PlanetName.Venus, PlanetName.Mercury, PlanetName.Moon];
    case Zodiac.Pisces:
      return [PlanetName.Saturn, PlanetName.Jupiter, PlanetName.Mars];
  }
}

export function detriment(z: Zodiac): PlanetName {
  switch (z) {
    case Zodiac.Aries:
      return PlanetName.Venus;
    case Zodiac.Taurus:
      return PlanetName.Mars;
    case Zodiac.Gemini:
      return PlanetName.Jupiter;
    case Zodiac.Cancer:
      return PlanetName.Saturn;
    case Zodiac.Leo:
      return PlanetName.Saturn;
    case Zodiac.Virgo:
      return PlanetName.Jupiter;
    case Zodiac.Libra:
      return PlanetName.Mars;
    case Zodiac.Scorpio:
      return PlanetName.Venus;
    case Zodiac.Sagittarius:
      return PlanetName.Mercury;
    case Zodiac.Capricorn:
      return PlanetName.Moon;
    case Zodiac.Aquarius:
      return PlanetName.Sun;
    case Zodiac.Pisces:
      return PlanetName.Mercury;
  }
}

export function fall(z: Zodiac): PlanetName|null {
  switch (z) {
    case Zodiac.Aries:
      return PlanetName.Saturn;
    case Zodiac.Taurus:
      return null;
    case Zodiac.Gemini:
      return null;
    case Zodiac.Cancer:
      return PlanetName.Mars;
    case Zodiac.Leo:
      return null;
    case Zodiac.Virgo:
      return PlanetName.Venus;
    case Zodiac.Libra:
      return PlanetName.Sun;
    case Zodiac.Scorpio:
      return PlanetName.Moon;
    case Zodiac.Sagittarius:
      return null;
    case Zodiac.Capricorn:
      return PlanetName.Jupiter;
    case Zodiac.Aquarius:
      return null;
    case Zodiac.Pisces:
      return PlanetName.Mercury;
  }
}

export function ptolemyTerm(z: Zodiac) {
  switch (z) {
    case Zodiac.Aries:
      return [{p:PlanetName.Jupiter,d:6}, {p:PlanetName.Venus,d:14}, {p:PlanetName.Mercury,d:21}, {p:PlanetName.Mars,d:26}, {p:PlanetName.Saturn,d:30}];
    case Zodiac.Taurus:
      return [{p:PlanetName.Venus,d:8}, {p:PlanetName.Mercury,d:15}, {p:PlanetName.Jupiter,d:22}, {p:PlanetName.Saturn,d:26}, {p:PlanetName.Mars,d:30}];
    case Zodiac.Gemini:
      return [{p:PlanetName.Mercury,d:7}, {p:PlanetName.Jupiter,d:14}, {p:PlanetName.Venus,d:21}, {p:PlanetName.Saturn,d:25}, {p:PlanetName.Mars,d:30}];
    case Zodiac.Cancer:
      return [{p:PlanetName.Mars,d:6}, {p:PlanetName.Jupiter,d:13}, {p:PlanetName.Mercury,d:20}, {p:PlanetName.Venus,d:27}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Leo:
      return [{p:PlanetName.Saturn,d:6}, {p:PlanetName.Mercury,d:13}, {p:PlanetName.Venus,d:19}, {p:PlanetName.Jupiter,d:25}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Virgo:
      return [{p:PlanetName.Mercury,d:7}, {p:PlanetName.Venus,d:13}, {p:PlanetName.Jupiter,d:18}, {p:PlanetName.Saturn,d:24}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Libra:
      return [{p:PlanetName.Saturn,d:6}, {p:PlanetName.Venus,d:11}, {p:PlanetName.Jupiter,d:19}, {p:PlanetName.Mercury,d:24}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Scorpio:
      return [{p:PlanetName.Mars,d:6}, {p:PlanetName.Jupiter,d:14}, {p:PlanetName.Venus,d:21}, {p:PlanetName.Mercury,d:27}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Sagittarius:
      return [{p:PlanetName.Jupiter,d:8}, {p:PlanetName.Venus,d:14}, {p:PlanetName.Mercury,d:19}, {p:PlanetName.Saturn,d:25}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Capricorn:
      return [{p:PlanetName.Venus,d:6}, {p:PlanetName.Mercury,d:12}, {p:PlanetName.Jupiter,d:19}, {p:PlanetName.Mars,d:25}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Aquarius:
      return [{p:PlanetName.Saturn,d:6}, {p:PlanetName.Mercury,d:12}, {p:PlanetName.Venus,d:20}, {p:PlanetName.Jupiter,d:25}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Pisces:
      return [{p:PlanetName.Venus,d:8}, {p:PlanetName.Jupiter,d:14}, {p:PlanetName.Mercury,d:20}, {p:PlanetName.Mars,d:26}, {p:PlanetName.Saturn,d:30}]
  }
}



// 埃及界
export function egyptianTerm(z: Zodiac) {
  switch (z) {
    case Zodiac.Aries:
      return [{p:PlanetName.Jupiter,d:6}, {p:PlanetName.Venus,d:12}, {p:PlanetName.Mercury,d:20}, {p:PlanetName.Mars,d:25}, {p:PlanetName.Saturn,d:30}];
    case Zodiac.Taurus:
      return [{p:PlanetName.Venus,d:8}, {p:PlanetName.Mercury,d:14}, {p:PlanetName.Jupiter,d:22}, {p:PlanetName.Saturn,d:27}, {p:PlanetName.Mars,d:30}];
    case Zodiac.Gemini:
      return [{p:PlanetName.Mercury,d:6}, {p:PlanetName.Jupiter,d:12}, {p:PlanetName.Venus,d:17}, {p:PlanetName.Mars,d:24}, {p:PlanetName.Saturn,d:30}];
    case Zodiac.Cancer:
      return [{p:PlanetName.Mars,d:7}, {p:PlanetName.Venus,d:13}, {p:PlanetName.Mercury,d:19}, {p:PlanetName.Jupiter,d:26}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Leo:
      return [{p:PlanetName.Jupiter,d:6}, {p:PlanetName.Venus,d:11}, {p:PlanetName.Saturn,d:18}, {p:PlanetName.Mercury,d:24}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Virgo:
      return [{p:PlanetName.Mercury,d:7}, {p:PlanetName.Venus,d:17}, {p:PlanetName.Jupiter,d:21}, {p:PlanetName.Mars,d:28}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Libra:
      return [{p:PlanetName.Saturn,d:6}, {p:PlanetName.Mercury,d:14}, {p:PlanetName.Jupiter,d:21}, {p:PlanetName.Venus,d:28}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Scorpio:
      return [{p:PlanetName.Mars,d:7}, {p:PlanetName.Venus,d:11}, {p:PlanetName.Mercury,d:19}, {p:PlanetName.Jupiter,d:24}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Sagittarius:
      return [{p:PlanetName.Jupiter,d:12}, {p:PlanetName.Venus,d:17}, {p:PlanetName.Mercury,d:21}, {p:PlanetName.Saturn,d:26}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Capricorn:
      return [{p:PlanetName.Mercury,d:7}, {p:PlanetName.Jupiter,d:14}, {p:PlanetName.Venus,d:22}, {p:PlanetName.Saturn,d:26}, {p:PlanetName.Mars,d:30}]
    case Zodiac.Aquarius:
      return [{p:PlanetName.Mercury,d:7}, {p:PlanetName.Venus,d:13}, {p:PlanetName.Jupiter,d:20}, {p:PlanetName.Mars,d:25}, {p:PlanetName.Saturn,d:30}]
    case Zodiac.Pisces:
      return [{p:PlanetName.Venus,d:12}, {p:PlanetName.Jupiter,d:16}, {p:PlanetName.Mercury,d:19}, {p:PlanetName.Mars,d:28}, {p:PlanetName.Saturn,d:30}]
  }
}
