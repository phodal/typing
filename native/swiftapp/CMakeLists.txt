cmake_minimum_required(VERSION 3.15.1)

project(AppDelegate LANGUAGES Swift)

#target_link_libraries(AppDelegate
#        "-framework AVFoundation"
#        "-framework CoreGraphics"
#        "-framework CoreMotion"
#        "-framework Foundation"
#        "-framework MediaPlayer"
#        "-framework OpenGLES"
#        "-framework QuartzCore"
#        "-framework UIKit"
#        )

add_executable(AppDelegate AppDelegate.swift)
