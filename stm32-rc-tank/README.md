```shell
cmake \
    -DBOARD_TYPE=core2 \
    -DPORT=stm32 \
    -DBOARD_VERSION=1.0.0 \
    -DHFRAMEWORK_PATH=/srv/hFramework \
    -GNinja ..

ninja
```
