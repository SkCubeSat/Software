add_subdirectory(Util)
# add_subdirectory(Core)

cmake_minimum_required (VERSION 3.13)

add_executable(${PROJECT_NAME}
"main.cpp"
)

target_compile_options(${PROJECT_NAME}
  PRIVATE
  -Wall
  -Werror
  -Wpedantic
  -O2
)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET ${PROJECT_NAME} 
  PROPERTY CXX_STANDARD 20)
endif()

target_link_libraries(${PROJECT_NAME} 
	PRIVATE
  	Core
	Util

  # external lib
  gpiodcxx
)
