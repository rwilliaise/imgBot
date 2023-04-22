cmake_minimum_required(VERSION 3.25)
project(imgbot)

set(THREADS_PREFER_PTHREADS_FLAG ON)

find_package(OpenCL)
find_package(libwebsockets)
find_package(CURL REQUIRED)
find_package(json-c REQUIRED)
find_package(Threads REQUIRED)

add_subdirectory(discord)
add_subdirectory(server)
