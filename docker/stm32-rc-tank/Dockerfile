FROM ubuntu:16.04

RUN apt-get update && apt-get install -y \
    git gcc-arm-none-eabi cmake libusb-1.0-0 g++ ninja-build

WORKDIR /srv/

# copy in hFramework
COPY stm32-rc-tank/hFramework /srv/hFramework

# build core2-flasher and place it in hFramework
RUN git clone --recurse-submodules https://github.com/husarion/flasher.git
WORKDIR /srv/flasher/build
RUN cmake ..
RUN make
RUN mv core2-flasher /srv/hFramework/tools/amd64-linux/core2-flasher

# copy in the code
COPY stm32-rc-tank/CMakeLists.txt /srv/CMakeLists.txt
COPY stm32-rc-tank/main.cpp /srv/main.cpp

# build the code
WORKDIR /srv/build
RUN ln -s /srv/hFramework hFramework
RUN cmake \
    -DBOARD_TYPE=core2 \
    -DPORT=stm32 \
    -DBOARD_VERSION=1.0.0 \
    -DHFRAMEWORK_PATH=./hFramework \
    -DHFRAMEWORK_DIR=./hFramework \
    -GNinja ..
RUN ninja

ENTRYPOINT ["bash", "-c", "mkdir -p /srv/artifacts && cp -f /srv/build/main.* /srv/artifacts"]

CMD []
