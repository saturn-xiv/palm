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

find_package(Boost REQUIRED COMPONENTS uuid date_time)
find_package(argparse CONFIG REQUIRED)
find_package(httplib CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(redis++ CONFIG REQUIRED)
find_package(SOCI CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(tomlplusplus CONFIG REQUIRED)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

# ---------------------------------------------------------

file(GLOB PROTOCOLS_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/protocols/src/*.cc)
file(GLOB TULIP_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(tulip ${PROTOCOLS_SOURCES} ${TULIP_SOURCES})
target_link_libraries(tulip PRIVATE m dl anl pthread
  Boost::uuid Boost::date_time
  argparse::argparse spdlog::spdlog_header_only cpr::cpr
  rabbitmq::rabbitmq-static redis++::redis++_static
  SOCI::soci_core_static SOCI::soci_postgresql_static
  nlohmann_json::nlohmann_json pantor::inja httplib::httplib tomlplusplus::tomlplusplus
  gRPC::grpc++
)
target_link_options(tulip PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(tulip PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(tulip PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${CMAKE_CURRENT_SOURCE_DIR}/protocols/include
)
