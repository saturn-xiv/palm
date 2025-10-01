cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  palm
  VERSION 2025.10.1
  DESCRIPTION "A printer daemon."
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
configure_file(../begonia/include/palm/version.h.in ${CMAKE_CURRENT_SOURCE_DIR}/../begonia/include/palm/version.hpp)

# ---------------------------------------------------------

add_subdirectory(../begonia ${CMAKE_BINARY_DIR}/begonia.out)

# ---------------------------------------------------------

file(GLOB PANSY_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(pansy ${PANSY_SOURCES})
target_link_libraries(pansy PRIVATE begonia)
target_link_options(pansy PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(pansy PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(pansy PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
