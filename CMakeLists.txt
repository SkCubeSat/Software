cmake_minimum_required(VERSION 3.13.4)

project(PBProject)

set(Executable_Name "testCan")
set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR arm)
set(CMAKE_CXX_COMPILER /usr/bin/arm-linux-gnueabihf-g++)

set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)


# Flags
set(CMAKE_CXX_FLAGS "-Wall -Wextra -g")

# Add your source files here
set(SOURCES
    ./src/drivers/testSockCanCpp.cpp

)

# Add your header files here
set(HEADERS

)

# Create an executable target
add_executable(${Executable_Name} ${SOURCES} ${HEADERS})


# Libraries

# Add the directory where your static library is located
target_link_directories(${Executable_Name} PRIVATE ./libs/libsockcanpp/)

# Link your executable with the static library
target_link_libraries(${Executable_Name} PRIVATE sockcanpp)

# Include the directory where library header files are located
target_include_directories(${Executable_Name} PRIVATE ./libs/libsockcanpp/include/)



