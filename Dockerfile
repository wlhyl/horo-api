FROM rust:1.94.1-alpine as build
WORKDIR /app

RUN sed -i s/dl-cdn.alpinelinux.org/mirror.tuna.tsinghua.edu.cn/g  /etc/apk/repositories
RUN apk add make musl-dev upx git

# 编译swe
RUN cd /tmp/ &&\
  git clone --depth 1 --filter=blob:none --sparse https://github.com/aloistr/swisseph.git &&\
  cd swisseph &&\
  make libswe.a && cp libswe.a /app

COPY ./ /app/

RUN echo '[source.crates-io]' > cargo.config
RUN echo 'replace-with = "ustc"' >> cargo.config
RUN echo [source.ustc] >> cargo.config
RUN echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> cargo.config
# RUN echo 'registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"' >> cargo.config

RUN RUSTFLAGS=-L/app cargo --config cargo.config install  --path api --root /tmp/app
# RUN CARGO_HTTP_MULTIPLEXING=false RUSTFLAGS=-L/app cargo --config cargo.config install  --path api --root /tmp/app
# RUN RUSTFLAGS=-L/app cargo install  --path api --root /tmp/app

# RUN RUSTFLAGS="-C target-feature=+crt-static" RUSTFLAGS=-L/app cargo --config cargo.config install  --path api --root /tmp/app

RUN strip -s /tmp/app/bin/horo_api
RUN strip  --strip-debug /tmp/app/bin/horo_api
RUN upx /tmp/app/bin/horo_api

FROM alpine

WORKDIR /app

COPY --from=build /tmp/app/bin/horo_api /app/bin/horo_api

# 创建非root用户并切换
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

EXPOSE 8080

ENTRYPOINT /app/bin/horo_api
