# core2_framework

## Build

TODO: Tidy this up (it's needlessly tied to the VSCode extension I stole it from and some stuff that only
applies to JetBrains IDEs)

```shell
cmake --build ./cmake-build-debug --target myproject.elf -j 12
cd ./cmake-build-debug
ln -s ../sdk sdk
ninja -f ./build.ninja
ninja -f ./build.ninja flash
```
