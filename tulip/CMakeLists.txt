cmake_minimum_required(VERSION 3.28)
include(ExternalProject)
include(FetchContent)

project(
    tulip
    VERSION 2025.10.25
    DESCRIPTION "A cryptographic thrift service(by Google Tink)."
    LANGUAGES C CXX)
# --------------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

# --------------------------------------------------------

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(include/tulip/version.h.in ${PROJECT_SOURCE_DIR}/include/tulip/version.hpp)

# --------------------------------------------------------

find_package(OpenSSL REQUIRED)

# --------------------------------------------------------

# https://github.com/gabime/spdlog
FetchContent_Declare(spdlog
  GIT_REPOSITORY  "https://github.com/gabime/spdlog.git"
  GIT_TAG         "v1.16.0"
)
# https://github.com/tink-crypto/tink-cc
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.4.0"
)

FetchContent_MakeAvailable(spdlog tink)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

add_library(tulip SHARED ${SOURCE_LIST})
set_target_properties(tulip PROPERTIES LINK_FLAGS_RELEASE -s)
# set_property(TARGET tulip PROPERTY POSITION_INDEPENDENT_CODE ON)

target_link_libraries(tulip
  ${OPENSSL_LIBRARIES}
  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
)

target_include_directories(tulip PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
    
  ${spdlog_SOURCE_DIR}/include
  $ENV{HOME}/.asdf/installs/erlang/28.1.1/usr/include
)
