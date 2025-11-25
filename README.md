# Horo API

## 项目简介

**Horo API** 是一个基于 Actix Web 框架开发的占星 Web 应用，提供本命盘、推运盘、七政四余等多种占星 API 服务。

## 功能概览

- 本命盘绘制
- 推运盘绘制
  - 法达
  - 小限
  - 行运
  - 日返
  - 月返
  - 日返、月返比较盘
- 七政四余盘星盘绘制
  - 没实现神煞
  - 没实现化曜
  - 没实现十二长生
  - 没实现纳音

## API 文档

完整接口文档请访问 Swagger UI 查看：
`http://localhost:8080/swagger-ui/`

## 快速开始

### 依赖要求

- Rust 1.88.0+
- 已编译的瑞士星历表（swisseph）及星历表数据文件
- Docker（可选）

### 本地运行

#### 环境准备

1. 下载并编译瑞士星历表：
   ```bash
   mkdir /path/to/swe
   cd /path/to/swe
   wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz
   tar xvzf swe.tar.gz
   cd swisseph-2.10.03
   make libswe.a
   ```
2. 下载星历表文件：
   ```bash
   cd /path/to/swe
   wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semo_18.se1
   wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semom48.se1
   wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/sepl_18.se1
   wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/seplm48.se1
   ```
3. 复制恒星文件：

   ```bash
   cp /path/to/horo_api/sefstars.txt /path/to/swe
   ```

4. 复制配置文件：

   ````bash
   cp .env.example .env          # 设置日志文件和星历表路径
   cp log4rs.yaml.example log4rs.yaml # 根据需要调整日志配置
    ```
   ````

#### 单元测试

```bash
RUSTFLAGS=-L/path/to/swe/src cargo test -- --test-threads 1
```

#### 开发环境启动 API 服务

```bash
  RUSTFLAGS=-L/path/to/swe/src \
  cargo run --features swagger,cors
```

### Docker 镜像构建

#### 构建镜像

```bash
docker build -t horo/api .
```

## 配置说明

### 环境变量

- **LOG4RS_CONFIG**: 日志配置文件路径，示例值：log4rs.yaml
- **EPHE_PATH**: 瑞士星历表及数据文件目录，示例值：/path/to/swe

### 可选 features

- `swagger`：API 文档
- `cors`：跨域支持

## 许可协议

本项目采用 AGPL-3.0 协议开源，详见 [LICENSE](LICENSE)。
