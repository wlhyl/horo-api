# Changelog

##[1.0.0] - 2026-04-01

### Added

- horo_date_time: 新增 `from_jd_ut1_zone` 方法，支持从 UT1 儒略日构造 HoroDateTime

### Fixed

- 修改因rust-sw变动导致的swe_calc_ut调用参数改变
- 修正星盘昼夜判断计算错误，根据太阳的地平坐标高度判断
- 修正horo_date_time的from_jd_ut1_zone函数计算jd_utc错误

### Changed

- 瑞士星历表的函数调用参数从jd_utc改为jd_ut
- 将项目依赖迁移到 workspace 统一管理
- 更新 Dockerfile：
  - 升级 Rust 版本到 1.94.1
  - swisseph 总是使用最新版本

##[0.4.0] - 2026-01-24

### Added

- 恒星增加：北河三、氐宿四

### Changed

- 恒星增加描述信息

##[0.3.1] - 2026-01-14

### Changed

- 比较盘添加比较盘的 12 宫头黄道经度

##[0.3.0] - 2026-01-04

### Added

- 本命盘增加恒星

##[0.2.3] - 2026-01-02

### Fixed

- Fix Bug [#2](https://github.com/wlhyl/horo-api/issues/2) 跨年日小限日期错误

## [0.2.2] - 2025-11-26

### Changed

- 协议修改为 AGPL-3.0

## [0.2.1] - 2025-08-14

### Added

- 增加福点计算功能
- 在比较盘和返照盘中包含福点数据

## [0.1.2] - 2025-08-07

### Added

- 增加每个宿的洞微大限开始时间

## [0.1.1] - 2025-07-16

### Changed

- 调整夜间法达周期中月交点的位置至七颗行星之后

## [0.1.0] - 2025-07-02

### Added

- 增加十干化曜功能
