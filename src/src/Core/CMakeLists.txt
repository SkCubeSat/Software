cmake_minimum_required (VERSION 3.26)

project ("Core" VERSION 0.1 LANGUAGES CXX DESCRIPTION "Core functionalities")

add_library(${PROJECT_NAME}

)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET ${PROJECT_NAME} 
  PROPERTY CXX_STANDARD 20)
endif()

target_include_directories(${PROJECT_NAME}
	PUBLIC
	"include"
)

target_link_libraries(${PROJECT_NAME}
PRIVATE
Util
)