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

# Enable C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH};${CMAKE_SOURCE_DIR}/cmake)

find_package(OpenGL REQUIRED)
find_package(GLFW 3.2 REQUIRED)
find_package(GLM REQUIRED)

include_directories(${CMAKE_SOURCE_DIR}/include)

set(CMAKE_CXX_FLAGS "-Wall -Wextra -Werror -Wno-deprecated")

add_library(roadkill STATIC
    src/io.cpp
    src/dump.cpp
    src/texturizer.cpp
    src/pixelmap.cpp
    src/viewport.cpp
    src/loader.cpp
    src/mesh.cpp)
target_include_directories(roadkill PRIVATE ${OPENGL_INCLUDE_DIR} ${GLM_INCLUDE_DIRS})

add_executable(dat src/dat.c)

add_executable(model src/model.cpp)
target_link_libraries(model roadkill)

add_executable(glook src/glook.cpp)
target_include_directories(glook PRIVATE ${GLFW_INCLUDE_DIR})
target_link_libraries(glook roadkill OpenGL::GL ${GLFW_LIBRARIES})

add_executable(pixdec src/pixdec.cpp)
target_link_libraries(pixdec roadkill)
