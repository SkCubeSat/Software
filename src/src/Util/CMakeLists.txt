cmake_minimum_required (VERSION 3.13)

project ("Util" VERSION 0.1 LANGUAGES CXX DESCRIPTION "Utilities")

add_library(${PROJECT_NAME}
"src/File.cpp"
)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET ${PROJECT_NAME} 
  PROPERTY CXX_STANDARD 20)
endif()

target_compile_options(${PROJECT_NAME}
  PRIVATE
  -Wall
  -Werror
  -Wpedantic
  -O2
)

target_include_directories(${PROJECT_NAME}
	PUBLIC
	"include"
)
