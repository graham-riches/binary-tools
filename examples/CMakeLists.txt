cmake_minimum_required(VERSION 3.1...3.15)
project(hello-world)
set(BINARY hello_world)

# set standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 11) 

# test specific pre-processor constants
add_definitions()

# add any additional includes
include_directories()

set(SOURCES
    hello_world.c
)

# add executable binary
add_executable(${BINARY} ${SOURCES})

# compiler flags
target_compile_options(${BINARY} PRIVATE
    -O2
)

# link libraries
target_link_libraries(${BINARY} PRIVATE)
