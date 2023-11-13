import { Zodiac } from '../type/enum/zodiac';
import { sin, cos, degNorm, zodiacLong, degreeToDMS } from './horo-math'

describe('horo-math', () => {
    describe('三角函数', () => {
        it('正弦', () => {
            let x = 84.1
            let y0 = sin(x)
            let y1 =Math.sin(x / 180 * Math.PI)
            expect(y1).toBeCloseTo(y1)
        })

        it('余弦', () => {
            let x = 84.1
            let y0 = cos(x)
            let y1 =Math.cos(x / 180 * Math.PI)
            expect(y1).toBeCloseTo(y1)
        })
    })

    describe('将度数调整到[0, 360)', () =>{
        let parameterizedTest = [
            2.0 - 360 * 3, 2.0 - 360 * 2, 2.0 - 360,
            2.0,
            2.0 + 360, 2.0 + 360 * 2, 2.0 + 360 * 3
        ]
        for(let param of parameterizedTest){
            it(`${param}度`, () => {
                expect(degNorm(param)).toEqual(2.0)
            })
        }
    })

    describe('度数化为：度、分、秒，对秒进行取整，不作四舍五入', () => {
        it("23度、12分、12秒", () => {
            let d = degreeToDMS(23+13/60.0+12.6/3600.0)
            expect(d.d).toEqual(23)
            expect(d.m).toEqual(13)
            expect(d.s).toEqual(12)
        })

        it("-23度、-12分、-12秒", () => {
            let d = degreeToDMS(-23 - 13/60.0 - 12.6/3600.0)
            expect(d.d).toEqual(-23)
            expect(d.m).toEqual(-13)
            expect(d.s).toEqual(-12)
        })
    })

    describe('得到给定黄道经度的星座度数', () =>{
        let parameterizedTest = []

        // [0,360)，每15度测试一次
        for(let i = 0;i<12;i++){
            parameterizedTest.push({long: i * 30 , zodiac: i, zodiacLong: 0})
            parameterizedTest.push({long: i * 30 + 15 , zodiac: i, zodiacLong: 15})
        }

        // 测试两组大于360
        parameterizedTest.push({long: 390 , zodiac: Zodiac.Taurus, zodiacLong: 0})
        parameterizedTest.push({long: 405 , zodiac: Zodiac.Taurus, zodiacLong: 15})

        // 测试四组负数
        parameterizedTest.push({long: -30 , zodiac: Zodiac.Pisces, zodiacLong: 0})
        parameterizedTest.push({long: -15 , zodiac: Zodiac.Pisces, zodiacLong: 15})
        parameterizedTest.push({long: -390 , zodiac: Zodiac.Pisces, zodiacLong: 0})
        parameterizedTest.push({long: -375 , zodiac: Zodiac.Pisces, zodiacLong: 15})

        for( let param of parameterizedTest){
            it(`${param.long}度`, () => {
                const long = zodiacLong(param.long)
                expect(long.zodiac).toEqual(param.zodiac)
                expect(long.long).toEqual(param.zodiacLong)
            })
        }
    })
  });