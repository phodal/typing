cmake_minimum_required(VERSION 3.15.1)

project(swiftapp LANGUAGES Swift)

set (SOURCES_FILES
        AppDelegate.swift
        )

add_executable(swiftapp MACOSX_BUNDLE ${SOURCES_FILES})
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -framework Cocoa")
