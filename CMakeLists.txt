cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  palm
  VERSION 2025.12.8
  DESCRIPTION "An open source online education & transaction solution."
  LANGUAGES CXX
)

# ---------------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# --------------------------------------------------------

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(begonia/include/palm/version.h.in ${PROJECT_SOURCE_DIR}/begonia/include/palm/version.hpp)

# ---------------------------------------------------------

add_subdirectory(begonia)
add_subdirectory(tulip)
add_subdirectory(tests)
