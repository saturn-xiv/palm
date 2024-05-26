cmake_minimum_required(VERSION 3.22)
include(FetchContent)

set(CMAKE_TOOLCHAIN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")

project(
    loquat
    VERSION 2024.5.2
    DESCRIPTION "A cryptographic rpc service(by google tink)."
    LANGUAGES C CXX)
# --------------------------------------------------------

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
configure_file(include/loquat/version.h.in ${PROJECT_SOURCE_DIR}/include/loquat/version.hpp)

# --------------------------------------------------------

find_package(OpenSSL REQUIRED)
find_package(argparse CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(Thrift CONFIG REQUIRED)

find_package(PkgConfig REQUIRED)
pkg_check_modules(thriftnb REQUIRED IMPORTED_TARGET thrift-nb)
pkg_check_modules(thriftz REQUIRED IMPORTED_TARGET thrift-z)

# --------------------------------------------------------

# https://github.com/tink-crypto/tink-cc
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.2.0"
)

FetchContent_MakeAvailable(tink)

# --------------------------------------------------------

add_subdirectory(gourd)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/src/*.cpp")

add_executable(loquat ${SOURCE_LIST})
set_target_properties(loquat PROPERTIES LINK_FLAGS_RELEASE -s)

target_link_libraries(loquat gourd
  OpenSSL::SSL OpenSSL::Crypto
  argparse::argparse
  spdlog::spdlog_header_only
  pantor::inja nlohmann_json::nlohmann_json
 
  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
  thrift::thrift PkgConfig::thriftz PkgConfig::thriftnb
)

target_include_directories(loquat PUBLIC
  ${PROJECT_SOURCE_DIR}/include
)


