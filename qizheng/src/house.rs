use crate::lunar_mansions::LunarMansionsName;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum HouseName {
    命,
    财,
    兄,
    田,
    子,
    奴,
    妻,
    疾,
    迁,
    官,
    福,
    相,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct ASCHouse {
    // @field:Schema(description = "命宫位置，0：戌，1: 酉，依此类推")
    //    @JsonProperty("first_house")
    // val firstHouse : Int,
    /// 上升点度数，没换算成上升点所在星座的度数
    asc_long: f64,
    /// 命度所在宿名
    xiu: LunarMansionsName,
    /// 入宿度数
    xiu_degree: f64,
    // @field:Schema(description = "命度的黄道经度")
    // val long :Double
}

impl ASCHouse {
    pub fn new(asc_long: f64, xiu: LunarMansionsName, xiu_degree: f64) -> Self {
        Self {
            asc_long,
            xiu,
            xiu_degree,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct House {
    /// 宫位的名称
    name: HouseName,

    /// 宫头的黄道经度
    pub long: f64,

    /// 宫头所在宿
    xiu: LunarMansionsName,

    /// 宫头入宿度
    xiu_degree: f64,
}

impl House {
    pub fn new(name: HouseName, long: f64, xiu: LunarMansionsName, xiu_degree: f64) -> Self {
        Self {
            name,
            long,
            xiu,
            xiu_degree,
        }
    }
}

// data class NaYingData(
//     @field:Schema(description = "年干支的纳音")
//     var naYin :String,
//     @field:Schema(description = "长生地支")
//     var zhangSheng :String
// )

// data class HoroscopeRequestData(
//     @field:Schema(example= "2021", description = "出生年份",required = true)
//     @field:NotNull(message = "缺少参数：year")
//     var year :Int?,

//     @field:Schema(example= "1", description = "出生月份", required = true)
//     @field:NotNull(message = "缺少参数：month")
//     @field:Range(min = 1, max = 31 ,message = "1<=month<=12")
//     var month :Int?,

//     @field:Schema(example= "1", description = "出生日", required = true)
//     @field:NotNull(message = "缺少参数：day")
//     @field:Range(min = 1, max = 31 ,message = "1<=day<=31")
//     var day :Int?,

//     @field:Schema(example= "12", description = "出生时", required = true)
//     @field:NotNull(message = "缺少参数：hour")
//     @field:Range(min = 0, max = 23 ,message = "0<=hour<24")
//     var hour :Int?,

//     @field:Schema(example= "0", description = "出生分", required = true)
//     @field:NotNull(message = "缺少参数：minute")
//     @field:Range(min = 0, max = 59 ,message = "1<=minute<=59")
//     var minute :Int?,

//     @field:Schema(example= "0", description = "出生秒", required = true)
//     @field:NotNull(message = "缺少参数：second")
//     @field:Range(min = 0, max = 60 ,message = "1<=second<=60")
//     var second :Int?,

//     @field:Schema(example= "8", description = "出生地时区，东区为正数，西区为负数", required = true)
//     @field:NotNull(message = "缺少参数：tz，tz：时区")
//     @field:Range(min = -12, max = 12 ,message = "-12<=tz<=12，tz：时区")
//     var tz : Int?,

//     @field:Schema(example= "false", description = "出生时的夏令时，有夏令时：true，无夏令时： false", required = true)
//     @field:NotNull(message = "缺少参数：st, st:夏令时")
//     var st :Boolean?,

//     @field:Schema(example= "16.42", description = "地理经度", required = true)
//     @field:DecimalMin(value = "-180", message = "-180<=地理经度=<180")
//     @field:DecimalMax(value = "180", message = "-180<=地理经度=<180")
//     @field:NotNull(message = "缺少参数：geo_long, geo_long: 地理经度")
//     var geoLong: Double?,

//     @field:Schema(example= "39.93", description = "地理纬度", required = true)
//     @field:DecimalMin(value = "-90", message = "-90<=地理纬度=<90")
//     @field:DecimalMax(value = "90", message = "-90<=地理纬度=<90")
//     @field:NotNull(message = "缺少参数：geo_lat, geo_lat: 地理纬度")
//     var geoLat: Double?,

//     @field:Schema(example= "2021", description = "推运年份",required = true)
//     @field:NotNull(message = "缺少参数：process_year")
//     var processYear :Int?,

//     @field:Schema(example= "1", description = "推运月份", required = true)
//     @field:NotNull(message = "缺少参数：process_month")
//     @field:Range(min = 1, max = 31 ,message = "1<=month<=12")
//     var processMonth :Int?,

//     @field:Schema(example= "1", description = "推运日", required = true)
//     @field:NotNull(message = "缺少参数：process_day")
//     @field:Range(min = 1, max = 31 ,message = "1<=day<=31")
//     var processDay :Int?,

//     @field:Schema(example= "12", description = "推运时", required = true)
//     @field:NotNull(message = "缺少参数：process_hour")
//     @field:Range(min = 0, max = 23 ,message = "0<=hour<24")
//     var processHour :Int?,

//     @field:Schema(example= "0", description = "推运分", required = true)
//     @field:NotNull(message = "缺少参数：process_minute")
//     @field:Range(min = 0, max = 59 ,message = "1<=minute<=59")
//     var processMinute :Int?,

//     @field:Schema(example= "0", description = "推运秒", required = true)
//     @field:NotNull(message = "缺少参数：process_second")
//     @field:Range(min = 0, max = 60 ,message = "1<=second<=60")
//     var processSecond :Int?,
// )
