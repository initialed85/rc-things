get_filename_component(PORT_DIR ${CMAKE_CURRENT_LIST_FILE} PATH) # for cmake before 2.8.3

set(prefix arm-none-eabi-)
if (WIN32)
  set(exesuffix .exe)
else()
  set(exesuffix "")
endif()

set(CMAKE_C_COMPILER ${prefix}gcc${exesuffix})
set(CMAKE_ASM_COMPILER ${prefix}gcc${exesuffix})
set(CMAKE_CXX_COMPILER ${prefix}g++${exesuffix})

enable_language(C CXX ASM)

set(CMAKE_AR ${prefix}ar${exesuffix}) # needs to be after enable_language
set(CMAKE_RANLIB ${prefix}ranlib${exesuffix})

set(port_flags "-fno-builtin-printf -fno-omit-frame-pointer -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mthumb -mfloat-abi=hard")
set(port_lflags "-T${HFRAMEWORK_DIR_Q}/stm32f4_linker.ld")

add_definitions(-DUSE_USB_OTG_FS -DUSE_STDPERIPH_DRIVER -DSTM32F4XX -DSTM32F4 -DSTM32F407ZG -D__FPU_USED -DARM_MATH_CM4 -D__FPU_PRESENT)

include_directories(${PORT_DIR}/include)
include_directories(${PORT_DIR}/src)
include_directories(${PORT_DIR}/src/hPeriph)

include_directories("${PORT_DIR}/src/hUSB/usb/")
include_directories("${PORT_DIR}/src/hUSB/")
