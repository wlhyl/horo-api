# 项目说明
该项目是一个基于 Actix Web 框架编写的占星 Web 应用，提供占星相关的 API 服务。主要功能包括：
- 本命盘绘制
- 推运盘绘制
  - 法达
  - 小限
  - 日返
  - 月返
  - 比较盘
- 七政四余盘星盘绘制
  - 没实现神煞
  - 没实现化曜
  - 没实现十二长生
  - 没实现纳音

# 运行前的环境准备
## 单元测试
* 下载瑞士星历表，并编译
    ```bash
    mkdir /path/to/swe
    cd /path/to/swe
    wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz
    tar xvzf swe.tar.gz
    cd swisseph-2.10.03
    make libswe.a
    ```

* 下载星历表文件
    ```bash
    cd /path/to/swe
    wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semo_18.se1
    wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semom48.se1
    wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/sepl_18.se1
    wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/seplm48.se1
    ```

* 将计算恒星的文件复制到/path/to/swe
```bash
cp /path/to/horo_api/sefstars.txt /path/to/swe
```

* 运行单元测试
    ```bash
    ephe_path=/path/to/swe RUSTFLAGS=-L/path/to/swe/src cargo test -- --test-threads 1
    ```



# 运行API服务

## 支持的features
* swagger: 启用swagger文档，访问地址：http://localhost:8080/swagger-ui/
* cors： 启用跨域支持

## 环境配置

### 日志配置
项目使用日志文件进行日志配置。
* 日志文件例子，文件名：log4rs.yaml，可修改为其它文件名。
```yaml
---
# 检查配置文件变动的时间间隔
refresh_rate: 30 seconds
# appender 负责将日志收集到控制台或文件, 可配置多个
appenders:
  stdout:
    kind: console
#  file:
#    kind: file
#    path: "log/log.log"
#    encoder:
#      # log 信息模式
#      pattern: "{d} - {m}{n}"
# 对全局 log 进行配置
root:
  level: info
  appenders:
    - stdout
#    - file # 启用此配置需要将上面的file节配置的注释取消，file与stdout平缓
```

## 运行API Server
```bash
LOG4RS_CONFIG=/path/to/log4rs.yaml \
  EPHE_PATH=/path/to/swe \
  RUSTFLAGS=-L/path/to/swe/src \
  cargo run --features swagger,cors
```

## API文档
启动服务后，可通过以下地址访问Swagger文档：
```
http://localhost:8080/swagger-ui/
```

# Docker支持
## 构建镜像
```bash
docker build -t horo/api .
```

## 运行容器
```bash
docker run -d -p 8080:8080 \
  -v /path/to/log4rs.yaml:/app/log4rs.yaml \
  -v /path/to/swe:/app/swe \
  -e LOG4RS_CONFIG=/app/log4rs.yaml \
  -e EPHE_PATH=/app/swe \
  --name horo-api \
  horo/api
```

## 注意事项
- 确保星历表文件目录已正确挂载
- 确保日志配置文件已正确挂载
- 容器内的路径与环境变量配置路径需保持一致

# 许可证
项目使用GPL-3.0 许可证 ([LICENSE](LICENSE))。