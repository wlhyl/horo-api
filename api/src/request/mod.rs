use horo::HouseName;
use serde::Deserialize;
#[cfg(feature = "swagger")]
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct HoroNativeRenReust {
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "出生年最小1900"))]
    pub year: i32,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub month: u8,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub day: u8,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub second: u8,
    /// 出生地时区，东区为正数，西区为负数
    #[validate(range(min = -12, max = 12, message = "-12<=时区<=12"))]
    pub tz: f64,
    /// 出生时的夏令时，有夏令时：true，无夏令时： false
    pub st: bool,

    /// 地理经度
    #[validate(range(min = -180, max = 180, message = "-180<=地理经度<=180"))]
    pub geo_long: f64,
    /// 地理纬度
    #[validate(range(min = -90, max = 90, message = "-90<=地理经度<=90"))]
    pub geo_lat: f64,
    /// 宫位系统，Alcabitus：阿卡比特
    pub house: HouseName,
}

// #[derive(Deserialize, Validate, ToSchema)]
// pub struct HoroNativeRenReust {
//     /// 年，最小值1900
//     #[validate(range(min = 1900))]
//     pub year: i32,
//     /// 月
//     #[validate(range(min = 1, max = 12))]
//     pub month: u8,
//     #[validate(range(min = 1, max = 31))]
//     /// 日
//     pub day: u8,
//     /// 时
//     #[validate(range(min = 0, max = 23))]
//     pub hour: u8,
//     /// 分
//     #[validate(range(min = 0, max = 59))]
//     pub minute: u8,
//     /// 秒
//     #[validate(range(min = 0, max = 59))]
//     pub second: u8,

//     /// 性别，男：true，女：false
//     pub masculine: bool,

//     /// 推运年，最小值1900
//     #[validate(range(min = 1900))]
//     pub process_year: i32,
//     /// 推运月
//     #[validate(range(min = 1, max = 12))]
//     pub process_month: u8,
//     #[validate(range(min = 1, max = 31))]
//     /// 推运日
//     pub process_day: u8,
//     /// 推运时
//     #[validate(range(min = 0, max = 23))]
//     pub process_hour: u8,
//     /// 推运分
//     #[validate(range(min = 0, max = 59))]
//     pub process_minute: u8,
//     /// 推运秒
//     #[validate(range(min = 0, max = 59))]
//     pub process_second: u8,
// }

#[derive(Deserialize, Validate)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct ProfectionRequest {
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "出生年最小1900"))]
    pub year: i32,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub month: u8,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub day: u8,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub second: u8,
    /// 出生地时区，东区为正数，西区为负数
    #[validate(range(min = -12, max = 12, message = "-12<=时区<=12"))]
    pub tz: f64,
    /// 出生时的夏令时，有夏令时：true，无夏令时： false
    pub st: bool,

    /// 推运年，最小值1900
    #[validate(range(min = 1900, message = "出生年最小1900"))]
    pub process_year: i32,
    /// 推运月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub process_month: u8,
    /// 推运日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub process_day: u8,
    /// 推运时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub process_hour: u8,
    /// 推运分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub process_minute: u8,
    /// 推运秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub process_second: u8,
}

#[derive(Deserialize, Validate)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct TransitRenReust {
    /// 年，最小值1900
    #[validate(range(min = 1900, message = "出生年最小1900"))]
    pub year: i32,
    /// 月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub month: u8,
    /// 日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub day: u8,
    /// 时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub hour: u8,
    /// 分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub minute: u8,
    /// 秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub second: u8,
    /// 出生地时区，东区为正数，西区为负数
    #[validate(range(min = -12, max = 12, message = "-12<=时区<=12"))]
    pub tz: f64,
    /// 出生时的夏令时，有夏令时：true，无夏令时： false
    pub st: bool,

    /// 地理经度
    #[validate(range(min = -180, max = 180, message = "-180<=地理经度<=180"))]
    pub geo_long: f64,
    /// 地理纬度
    #[validate(range(min = -90, max = 90, message = "-90<=地理经度<=90"))]
    pub geo_lat: f64,
    /// 宫位系统，Alcabitus：阿卡比特
    pub house: HouseName,

    /// 推运年，最小值1900
    #[validate(range(min = 1900, message = "出生年最小1900"))]
    pub process_year: i32,
    /// 推运月
    #[validate(range(min = 1, max = 12, message = "1<=月份<=12"))]
    pub process_month: u8,
    /// 推运日
    #[validate(range(min = 1, max = 31, message = "1<=日期<=31"))]
    pub process_day: u8,
    /// 推运时
    #[validate(range(min = 0, max = 23, message = "0<=时<=23"))]
    pub process_hour: u8,
    /// 推运分
    #[validate(range(min = 0, max = 59, message = "0<=分<=59"))]
    pub process_minute: u8,
    /// 推运秒
    #[validate(range(min = 0, max = 59, message = "0<=秒<=59"))]
    pub process_second: u8,
}
