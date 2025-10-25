cmake_minimum_required(VERSION 3.28)
include(ExternalProject)
include(FetchContent)

project(
    marigold
    VERSION 2025.10.25
    DESCRIPTION "An erlang NIFs library for Minio SDK."
    LANGUAGES C CXX)
# --------------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# --------------------------------------------------------

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(include/marigold/version.h.in ${PROJECT_SOURCE_DIR}/include/marigold/version.hpp)

# --------------------------------------------------------

find_package(miniocpp CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(tomlplusplus CONFIG REQUIRED)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

add_library(marigold SHARED ${SOURCE_LIST})
set_target_properties(marigold PROPERTIES LINK_FLAGS_RELEASE -s)

target_link_libraries(marigold
  ${OPENSSL_LIBRARIES}
  miniocpp::miniocpp spdlog::spdlog_header_only tomlplusplus::tomlplusplus
)

target_include_directories(marigold PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  
  $ENV{HOME}/.asdf/installs/erlang/28.1.1/usr/include
)
