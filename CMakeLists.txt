cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  palm
  VERSION 2025.6.29
  DESCRIPTION "An open source online education and transaction solution."
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
configure_file(begonia/include/basil/version.h.in ${PROJECT_SOURCE_DIR}/begonia/include/basil/version.hpp)

# ---------------------------------------------------------

add_subdirectory(begonia)
add_subdirectory(bamboo)
add_subdirectory(lavender)
add_subdirectory(apps)
add_subdirectory(tests)
