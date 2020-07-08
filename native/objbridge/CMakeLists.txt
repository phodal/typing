cmake_minimum_required(VERSION 3.15.1)

project(objbridge)

set (CMAKE_CXX_STANDARD 11)
set(CMAKE_C_FLAGS "-x objective-c")

add_library(objbridge STATIC bridge.mm bridge.h AppDelegate.h AppDelegate.mm)

install(TARGETS objbridge DESTINATION .)
