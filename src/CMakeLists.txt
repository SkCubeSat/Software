cmake_minimum_required(VERSION 3.26)
project(src)

set(CMAKE_CXX_STANDARD 20)

include_directories(classes)

add_executable(src
        classes/CANDevice.cpp
        classes/CANDevice.h
        classes/I2CDevice.h
        classes/SPIDevice.h
        main.cpp
        classes/Util/GPIO/GPIO.h
        classes/Util/GPIO/GPIO.cpp)
