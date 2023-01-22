cmake_minimum_required(VERSION 2.8)
project(ARoboCoreProject NONE)

set(HFRAMEWORK_PATH ./sdk)
set(BOARD_TYPE CORE2)
set(PORT STM32)
set(BOARD_VERSION 1.0.0)

include_directories(/usr/include)
include_directories(./sdk/ports/stm32/include)
include_directories(./sdk/ports/stm32/src)
include_directories(./sdk/ports/stm32/src/hPeriph)
include_directories(./sdk/ports/stm32/src/hUSB/usb)
include_directories(./sdk/ports/stm32/src/hUSB)
include_directories(./sdk/include)
include_directories(./sdk/include/hCloudClient)
include_directories(./sdk/include/hROS)

include(${HFRAMEWORK_PATH}/hFramework.cmake)

enable_module(hCloudClient)
enable_module(hSensors)
enable_module(hModules)
enable_module(hROS)

add_hexecutable(myproject main.cpp)
