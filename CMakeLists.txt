cmake_minimum_required(VERSION 3.22)

project(Stantz VERSION 0.1)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)


# External libs
# =============================================================================

# Inclue Eigen3
find_package(Eigen3 3.4 REQUIRED NO_MODULE)
include_directories(${EIGEN3_INCLUDE_DIRS})

# Include SDL2
find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})


# Internal libs
# =============================================================================

# Stantz lib
add_library(
    stantz STATIC
    src/geometry.cpp
    src/rendering.cpp)
target_include_directories(
    stantz PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include/stantz)
target_include_directories(
    stantz PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(
    stantz PRIVATE
    Eigen3::Eigen
    ${SDL2_LIBRARIES})


# Executables
# =============================================================================

# Example
add_executable(test examples/test.cpp)
target_link_libraries(test PRIVATE stantz)
