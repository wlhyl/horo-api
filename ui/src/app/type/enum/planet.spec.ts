import { PlanetName as Planet } from './planet';

describe('Planet 枚举', () => {
  it('太阳', () => {
    expect(Planet.Sun).toBe('Sun');
  });

  it('月亮', () => {
    expect(Planet.Moon).toBe('Moon');
  });

  it('水星', () => {
    expect(Planet.Mercury).toBe('Mercury');
  });

  it('金星', () => {
    expect(Planet.Venus).toBe('Venus');
  });

  it('火星', () => {
    expect(Planet.Mars).toBe('Mars');
  });

  it('火星', () => {
    expect(Planet.Jupiter).toBe('Jupiter');
  });

  it('土星', () => {
    expect(Planet.Saturn).toBe('Saturn');
  });

  //   it('天王星', () => {
  //     expect(Planet.SE_URANUS).toBe(7);
  //   });

  //   it('海王星', () => {
  //     expect(Planet.SE_NEPTUNE).toBe(8);
  //   });

  //   it('冥王星', () => {
  //     expect(Planet.SE_PLUTO).toBe(9);
  //   });

  //   it('平北交', () => {
  //     expect(Planet.SE_MEAN_NODE).toBe(10);
  //   });

  //   it('真北交', () => {
  //     expect(Planet.SE_TRUE_NODE).toBe(11);
  //   });

  //   it('平月孛', () => {
  //     expect(Planet.SE_MEAN_APOG).toBe(12);
  //   });

  //   it('真月孛', () => {
  //     expect(Planet.SE_OSCU_APOG).toBe(13);
  //   });

  //   it('上升点', () => {
  //     expect(Planet.HORO_ASC).toBe(-100);
  //   });

  //   it('中天', () => {
  //     expect(Planet.HORO_MC).toBe(-101);
  //   });

  //   it('下降点', () => {
  //     expect(Planet.HORO_DSC).toBe(-102);
  //   });

  //   it('天底', () => {
  //     expect(Planet.HORO_IC).toBe(-103);
  //   });

  it('北交点', () => {
    expect(Planet.NorthNode).toBe('NorthNode');
  });

  it('南交点', () => {
    expect(Planet.NorthNode).toBe('NorthNode');
  });
});
