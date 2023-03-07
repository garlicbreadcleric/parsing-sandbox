cmake_minimum_required(VERSION 3.10)

project(simdutf_wrapper)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(simdutf_wrapper SHARED simdutf_wrapper.cpp)

add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../libsimdutf ${CMAKE_CURRENT_BINARY_DIR}/libsimdutf)

target_include_directories(simdutf_wrapper PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../simdutf/include)

target_link_libraries(simdutf_wrapper simdutf)