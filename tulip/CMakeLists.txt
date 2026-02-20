cmake_minimum_required(VERSION 3.28)
include(FetchContent)

project(
  tulip
  VERSION 2026.2.20
  DESCRIPTION "An open source online education & transaction solution."
  LANGUAGES CXX
)
# -----------------------------------------------------------------------------
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fuse-ld=mold")
# -----------------------------------------------------------------------------
add_subdirectory(gourd)
add_subdirectory(tests)
# -----------------------------------------------------------------------------
file(GLOB TULIP_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(tulip ${TULIP_SOURCES})
target_link_libraries(tulip PRIVATE gourd)
target_link_options(tulip PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(tulip PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(tulip PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
