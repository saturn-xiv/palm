cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  pansy
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
configure_file(include/pansy/version.h.in ${CMAKE_CURRENT_SOURCE_DIR}/include/pansy/version.hpp)

# ---------------------------------------------------------

find_package(argparse CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(httplib CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(boost_uuid CONFIG REQUIRED)
find_package(boost_algorithm CONFIG REQUIRED)
find_package(protobuf CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)
find_package(PkgConfig REQUIRED)


pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

# ---------------------------------------------------------

file(GLOB PANSY_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)
file(GLOB GOURD_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/../gourd/src/cups.*.cc)

add_executable(pansy ${PANSY_SOURCES} ${GOURD_SOURCES})
target_link_libraries(pansy PUBLIC m dl anl cups
  argparse::argparse spdlog::spdlog_header_only
  httplib::httplib nlohmann_json::nlohmann_json pantor::inja  
  PkgConfig::tomlplusplus 
  Boost::uuid 
  protobuf::libprotobuf gRPC::grpc++  
)
target_link_options(pansy PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(pansy PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(pansy PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/../gourd/include
  ${CMAKE_CURRENT_SOURCE_DIR}/include  
)
