import { Zodiac } from '../type/enum/zodiac';

/**
 *
 * @param x 单位：度
 * @returns 正弦值
 */
export function sin(x: number): number {
  return Math.sin((x * Math.PI) / 180);
}

/**
 *
 * @param x 单位：度
 * @returns 余弦值
 */
export function cos(x: number): number {
  return Math.cos((x * Math.PI) / 180);
}

// /**
//  *
//  * @param x [-1, 1]
//  * @returns 反正弦，单位：度
//  */
// export function asin(x :number) :number{
//     return Math.asin(x) * 180 / Math.PI
// }

/**
 *
 * @param x 度数
 * @returns 将度数调整到[0, 360)
 */
export function degNorm(x: number): number {
  while (x < 0) x += 360;
  while (x >= 360) x -= 360;
  return x;
}

/**
 *
 * @param degree 度数
 * @returns 返回度数的，度、分、秒，秒不作四舍五入
 */
export function degreeToDMS(degree: number): {
  d: number;
  m: number;
  s: number;
} {
  let l = Math.abs(degree);
  let d = Math.floor(l);
  let m = (l - d) * 60;
  let s = (m - Math.floor(m)) * 60;
  s = Math.floor(s);
  m = Math.floor(m);

  if (degree < 0) {
    d = -d;
    m = -m;
    s = -s;
  }
  return { d: d, m: m, s: s };
}

/**
 * 得到给定黄道经度的星座度数
 * @param d 黄道经度
 * @returns 星座id和星座上的度数
 */
export function zodiacLong(d: number): { zodiac: Zodiac; long: number } {
  let x = degNorm(d);
  let n = Math.floor(x / 30);

  return { zodiac: n, long: x - n * 30 };
}

/**
 * NewtonIteration 牛顿迭代法求解方程的根
 */
export function newtonIteration(
  init_value: number,
  f: (x: number) => number
): number {
  const epsilon = 1e-7;
  const delta = 5e-6;
  let x = 0.0;
  let x0 = init_value;

  for (let i = 0; i < 1000; i++) {
    x = x0;
    const fx = f(x);

    // 导数
    const fx_delta = f(x + delta);

    const fpx = (fx_delta - fx) / delta;
    x0 = x - fx / fpx;
    if (Math.abs(x0 - x) <= epsilon) {
      break;
    }
  }
  if (Math.abs(x0 - x) <= epsilon) {
    return x;
  } else {
    throw '1000次迭代，求解失败，调整初值重试';
  }
}
