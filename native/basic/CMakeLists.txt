cmake_minimum_required(VERSION 3.15.1)

project(doubler)

set(SRC_FILES doubler.c)
add_library(doubler STATIC ${SRC_FILES})
install(TARGETS doubler DESTINATION .)

#target_link_libraries(${PROJECT_NAME} pthread)
