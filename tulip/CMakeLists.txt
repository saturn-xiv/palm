cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  tulip
  VERSION 2026.2.13
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
configure_file(include/tulip/version.h.in ${PROJECT_SOURCE_DIR}/include/tulip/version.hpp)

# ---------------------------------------------------------



# ---------------------------------------------------------

file(GLOB PROTOCOLS_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/protocols/src/*.cc)
file(GLOB TULIP_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(tulip ${PROTOCOLS_SOURCES} ${TULIP_SOURCES})
target_link_libraries(tulip PRIVATE begonia)
target_link_options(tulip PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(tulip PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(tulip PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${CMAKE_CURRENT_SOURCE_DIR}/protocols/include
)
