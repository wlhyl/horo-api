**rust编写的基于actix web的占星web app**

# api
## 单元测试

* 下载瑞士星历表，并编译
```bash
mkdir /tmp/swe
cd /tmp/swe
wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz
tar xvzf swe.tar.gz
cd swisseph-2.10.03
make libswe.a
```

* 下载星历表文件
```bash
cd /tmp/swe
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semo_18.se1
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/semom48.se1
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/sepl_18.se1
wget https://raw.githubusercontent.com/aloistr/swisseph/master/ephe/ephe/seplm48.se1
```

* 单元测试
```bash
ephe_path=/tmp/swe RUSTFLAGS=-L/tmp/swe/src cargo test -- --test-threads 1
```

## api server支持的features
* swagger: 启用swagger，访问地址：http://localhost:8080/swagger-ui/
* cors： 启用跨域

## 运行api
```bash
ephe_path=/tmp/swe RUSTFLAGS=-L/tmp/swe/src cargo run  --features swagger,cors
```

## 运行ui
运行ui需要Node.js，请先安装Node.js>=v18.17.1
```bash
npm install -g @ionic/cli
cd ui
ionic serve
```

# 构建镜像
* api
```bash
docker build -t horo/api . 
```

* ui
```bash
cd ui
docker build -t horo/ui .
```

# 部署
http访问，cert-manager.io/cluster-issuer 注解可以不用设置。

如果启用https访问，将ingress.tls设置为true，
```bash
helm install horo chart \
  --namespace horo \
  --create-namespace \
  --set ingress.enabled=true \
  --set ingress.className=nginx \
  --set ingress.hostname=your_hostname \
  --set ingress.tls=false \
  --set ingress.annotations."cert-manager\.io/cluster-issuer"=your_issuer
```
