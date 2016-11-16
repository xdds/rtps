cmake_minimum_required (VERSION 2.6)
project (PingPong)

add_executable(ping_pong main.cxx)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../install/include)
link_directories(${CMAKE_CURRENT_SOURCE_DIR}/../install/lib)

add_definitions(
    -std=c++11 # Or -std=c++0x
    # Other flags
)