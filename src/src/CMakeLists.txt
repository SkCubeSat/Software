add_subdirectory(Util)
# add_subdirectory(Core)

cmake_minimum_required (VERSION 3.26)

add_executable(${PROJECT_NAME}
"main.cpp"
)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET ${PROJECT_NAME} 
  PROPERTY CXX_STANDARD 20)
endif()

target_link_libraries(${PROJECT_NAME} 
	PRIVATE
	gpiod
	# Core
	Util
)
