cmake_minimum_required(VERSION 3.22)

project(Stantz VERSION 0.1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Include SDL2
find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

# LinAlg lib
# TODO: Replace with Eigen
add_library(linalg src/linalg.cpp)
target_include_directories(
    linalg PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include)

# Stantz lib
add_library(
    stantz STATIC
    src/geometry.cpp
    src/lights.cpp
    src/objects.cpp
    src/rendering.cpp)
target_include_directories(
    stantz PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include/stantz)
target_include_directories(
    stantz PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(stantz PRIVATE ${SDL2_LIBRARIES})
target_link_libraries(stantz PRIVATE linalg)

# Example
add_executable(test examples/test.cpp)
target_link_libraries(test PRIVATE stantz)
target_link_libraries(test PRIVATE linalg)
