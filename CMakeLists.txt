

set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)

cmake_minimum_required(VERSION 3.0)
MESSAGE(STATUS "======================")
MESSAGE(STATUS "Starting lnGdb ")
MESSAGE(STATUS "======================")

SET(LN_ENABLE_USBD True  CACHE INTERNAL "")
SET(LN_ENABLE_I2C  FALSE CACHE INTERNAL "")
SET(LN_ENABLE_SPI  True  CACHE INTERNAL "")
SET(LN_ENABLE_RUST True  CACHE INTERNAL "")

SET(AF_FOLDER  ${CMAKE_SOURCE_DIR}/lnArduino/)
include(./mcuSelect.cmake)
SET(CMAKE_TOOLCHAIN_FILE ${AF_FOLDER}/lnArduino.cmake)

PROJECT(lnGdb NONE) # the NONE is very important !

enable_language(C CXX ASM) # this is important too!

#

include_directories(${CMAKE_SOURCE_DIR})
include_directories(${CMAKE_SOURCE_DIR}/include)


add_subdirectory(${AF_FOLDER})

include(${AF_FOLDER}/setup.cmake)
USE_LIBRARY(tinyUsb)

include(${AF_FOLDER}/libraries/tinyUsb/tiny.cmake)

add_subdirectory(packages)

GENERATE_GD32_FIRMWARE(lnGdb src/main.cpp)
target_link_libraries(lnGdb gd32_usb_usbd rnGdbStub rnArduino)    
