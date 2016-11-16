cmake_minimum_required (VERSION 2.6)
project (PingPong)

add_executable(ping_pong main.cxx)

link_directories(${CMAKE_CURRENT_SOURCE_DIR}/../install/lib)
