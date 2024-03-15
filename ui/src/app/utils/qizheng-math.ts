import { degNorm } from './horo-math';

/**
 * 得到给定黄道经度的星座度数
 * @param d 黄道经度
 * @returns 星座id和星座上的度数
 */
export function zodiacLong(d: number): { zodiac: string; long: number } {
  const houses = [
    '戌',
    '酉',
    '申',
    '未',
    '午',
    '巳',
    '辰',
    '卯',
    '寅',
    '丑',
    '子',
    '亥',
  ];
  let x = degNorm(d);
  let n = Math.floor(x / 30);

  return { zodiac: houses[n], long: x - n * 30 };
}
