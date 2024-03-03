**用于占星 app 的时间 lib**

## 单元测试

- 下载瑞士星历表，并编译

```bash
mkdir /tmp/swe
cd /tmp/swe
wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz
tar xvzf swe.tar.gz
cd swisseph-2.10.03
make libswe.a
```

- 单元测试

```bash
RUSTFLAGS=-L/tmp/swe/src cargo test
```
