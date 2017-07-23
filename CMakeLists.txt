#
# Part of Roadkill Project. Check http://<urlhere> for latest version.
#
# Copyright 2010, Stanislav Karchebnyy <berkus@madfire.net>
#
# Distributed under the Boost Software License, Version 1.0.
# (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
#
cmake_minimum_required(VERSION 3.5.1 FATAL_ERROR)
project(Carma VERSION 0.0.1)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH};${CMAKE_SOURCE_DIR}/cmake)

find_package(OpenGL REQUIRED)
find_package(GLFW 3.2 REQUIRED)

include_directories(${CMAKE_SOURCE_DIR})

set(CMAKE_CXX_FLAGS "-Wall -Wextra -Werror -Wno-deprecated")

add_library(roadkill STATIC
    io.cpp dump.cpp texturizer.cpp pixelmap.cpp viewport.cpp loader.cpp mesh.cpp)
target_include_directories(roadkill PRIVATE ${OPENGL_INCLUDE_DIR})

add_executable(dat dat.c)

add_executable(model model.cpp)
target_link_libraries(model roadkill)

add_executable(glook glook.cpp)
target_include_directories(glook PRIVATE ${GLFW_INCLUDE_DIR})
target_link_libraries(glook roadkill OpenGL::GL OpenGL::GLU ${GLFW_LIBRARIES})

add_executable(pixdec pixdec.cpp)
target_link_libraries(pixdec roadkill)
