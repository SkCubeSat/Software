cmake_minimum_required(VERSION 3.26)
project(src)

set(CMAKE_CXX_STANDARD 20)

include_directories(src)

add_executable(src
        src/CANDevice.cpp
        src/CANDevice.h
        src/I2CDevice.h
        src/SPIDevice.h
        main.cpp
        src/Core/include/GPIO.h
        src/Core/src/GPIO.cpp
        src/Util/File.h
        src/Util/File.cpp
        src/Core/include/Core.h)
