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

find_package(OpenGL REQUIRED)
find_package(glfw3 3.2 REQUIRED)

include_directories(${CMAKE_SOURCE_DIR})

set(CMAKE_CXX_FLAGS "-Wall -Wextra -Werror")

add_library(roadkill STATIC
    io.cpp dump.cpp texturizer.cpp pixelmap.cpp viewport.cpp loader.cpp mesh.cpp)
target_include_directories(roadkill PRIVATE ${OPENGL_INCLUDE_DIR} ${GLFW_INLUDE_DIRS})

add_executable(dat dat.c)

add_executable(model model.cpp)
target_link_libraries(model roadkill)

add_executable(glook glook.cpp)
target_link_libraries(glook roadkill OpenGL::GL)
target_link_libraries(glook ${OPENGL_gl_LIBRARY})

add_executable(pixdec pixdec.cpp)
target_link_libraries(pixdec roadkill)
