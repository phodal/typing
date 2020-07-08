cmake_minimum_required(VERSION 3.15.1)

project(objchello)

set (CMAKE_CXX_STANDARD 11)
set(CMAKE_C_FLAGS "-x objective-c")

add_executable(objchello AppDelegate.mm)
