cmake_minimum_required(VERSION 3.15.1)

enable_language(Swift)

project(swiftapp LANGUAGES Swift)

set (CMAKE_CXX_STANDARD 11)
set(CMAKE_C_FLAGS "-x objective-c")

set (SOURCES_FILES
        AppDelegate.swift
        )

set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -framework Cocoa")

set(MACOSX_FRAMEWORK_IDENTIFIER "com.phodal")
set(MACOSX_FRAMEWORK_NAME "typing")
set(MACOSX_FRAMEWORK_BUNDLE_VERSION "1.0.0")
set(MACOSX_FRAMEWORK_SHORT_VERSION_STRING "1.0.0")

add_executable(swiftapp MACOSX_BUNDLE ${SOURCES_FILES})
