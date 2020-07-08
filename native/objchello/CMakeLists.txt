cmake_minimum_required(VERSION 3.15.1)

project(objchello)

set (CMAKE_CXX_STANDARD 11)
set(CMAKE_C_FLAGS "-x objective-c")


set (SOURCES_FILES
        AppDelegate.mm
        AppDelegate.h
        )



add_library(objchello AppDelegate.h AppDelegate.mm)
#add_executable(objchello MACOSX_BUNDLE ${SOURCES_FILES})
