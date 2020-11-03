cmake_minimum_required(VERSION 3.17)
project(kjsl_cpp_test)

set(CMAKE_CXX_STANDARD 14)

add_executable(kjsl_cpp_test main.cpp)
target_link_libraries(kjsl_cpp_test ${CMAKE_SOURCE_DIR}/../kjsl_c_lib/cmake-build-debug/libkjsl_bind.a)