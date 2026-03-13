cmake_minimum_required(VERSION 3.28)
include(FetchContent)

project(
  lavender
  VERSION 2026.3.13
  DESCRIPTION "A logging crawler."
  LANGUAGES CXX
)
# -----------------------------------------------------------------------------
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fuse-ld=mold")
# -----------------------------------------------------------------------------
execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(include/lavender/version.h.in ${CMAKE_CURRENT_SOURCE_DIR}/include/lavender/version.hpp)
# -----------------------------------------------------------------------------
FetchContent_Declare(spdlog
  GIT_REPOSITORY "https://github.com/gabime/spdlog.git"
  GIT_TAG        "v1.17.0"
)
FetchContent_Declare(tomlplusplus
  GIT_REPOSITORY "https://github.com/marzer/tomlplusplus.git"
  GIT_TAG        "v3.4.0"
)
FetchContent_Declare(nlohmann_json
  GIT_REPOSITORY "https://github.com/nlohmann/json.git"
  GIT_TAG        "v3.12.0"
)
FetchContent_Declare(cpr
  GIT_REPOSITORY "https://github.com/libcpr/cpr.git"
  GIT_TAG        "1.14.2"
)

FetchContent_MakeAvailable(spdlog tomlplusplus nlohmann_json cpr)
# -----------------------------------------------------------------------------
file(GLOB LAVENDER_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(lavender ${LAVENDER_SOURCES})
target_link_options(lavender PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(lavender PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(lavender PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
