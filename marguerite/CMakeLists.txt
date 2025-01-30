cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  marguerite
  VERSION 2025.1.17
  DESCRIPTION "NIFs for Thistle."
  LANGUAGES CXX
)

# ---------------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXE_LINKER_FLAGS "-s -Wl,--build-id=sha1")

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
# set(CMAKE_CXX_FLAGS  "${CMAKE_CXX_FLAGS} -fPIC" )
# set(CMAKE_C_FLAGS  "${CMAKE_C_FLAGS} -fPIC" )
# --------------------------------------------------------

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(include/marguerite/version.h.in ${PROJECT_SOURCE_DIR}/include/marguerite/version.hpp)

# ---------------------------------------------------------
# find_package(ZLIB CONFIG REQUIRED)
find_package(OpenSSL CONFIG REQUIRED)
find_package(miniocpp CONFIG REQUIRED)
find_package(unofficial-sodium CONFIG REQUIRED)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.3.0"
)
FetchContent_MakeAvailable(tink)

# ---------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

add_library(marguerite SHARED ${SOURCE_LIST})
# target_link_options(marguerite PUBLIC "")
set_target_properties(marguerite PROPERTIES LINK_FLAGS_RELEASE -s)

target_link_libraries(marguerite
  ZLIB::ZLIB OpenSSL::SSL OpenSSL::Crypto
  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
  unofficial-sodium::sodium miniocpp::miniocpp
)


target_include_directories(marguerite PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
    
  ${spdlog_SOURCE_DIR}/include
)

