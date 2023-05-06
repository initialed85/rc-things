cmake_minimum_required(VERSION 2.8.12)

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_VERSION "0.0.0")

set(HFRAMEWORK_PATH ./hFramework)
set(BOARD_TYPE CORE2)
set(PORT STM32)
set(BOARD_VERSION 1.0.0)

project(stm32-rc-tank NONE)

include(${HFRAMEWORK_PATH}/hFramework.cmake)

enable_module(hSensors)
enable_module(hModules)

add_hexecutable(main main.cpp)
