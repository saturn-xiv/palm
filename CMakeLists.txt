cmake_minimum_required(VERSION 3.16)
include(ExternalProject)
include(FetchContent)

project(
    palm
    VERSION 0.1.0
    DESCRIPTION "A total free education & translation & ops solution."
    LANGUAGES C CXX)


include(docker/third.cmake)
configure_file(include/palm/version.h.in ${PROJECT_SOURCE_DIR}/include/palm/version.hpp)

add_subdirectory(src)
add_subdirectory(apps)
add_subdirectory(tests)
