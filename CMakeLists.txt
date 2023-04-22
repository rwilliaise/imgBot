cmake_minimum_required(VERSION 3.25)
project(imgbot)

find_package(libwebsockets)
find_package(CURL REQUIRED)
find_package(json-c REQUIRED)

add_subdirectory(discord)
