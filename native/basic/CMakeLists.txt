cmake_minimum_required(VERSION 3.15.1)

project(doubler C)

set(SRC_FILES doubler.c)
add_library(doubler STATIC ${SRC_FILES})
install(TARGETS doubler DESTINATION .)
