cmake_minimum_required(VERSION 3.9)
project(clipper CXX)

add_library(clipper STATIC wrapper.cpp clipper.cpp)

install(TARGETS clipper DESTINATION .)