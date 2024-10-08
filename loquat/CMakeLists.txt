cmake_minimum_required(VERSION 3.28)
include(ExternalProject)
include(FetchContent)

project(
    loquat
    VERSION 2023.7.21
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

# --------------------------------------------------------

FetchContent_Declare(spdlog
  GIT_REPOSITORY  "https://github.com/gabime/spdlog.git"
  GIT_TAG         "v1.14.1"
)
FetchContent_Declare(tomlplusplus
  GIT_REPOSITORY  "https://github.com/marzer/tomlplusplus.git"
  GIT_TAG         "v3.4.0"
)
FetchContent_Declare(argparse
  GIT_REPOSITORY  "https://github.com/p-ranav/argparse.git"
  GIT_TAG         "v3.1"
)
FetchContent_Declare(inja
  GIT_REPOSITORY  "https://github.com/pantor/inja.git"
  GIT_TAG         "v3.4.0"
)
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.2.0"
)
FetchContent_MakeAvailable(spdlog argparse tomlplusplus inja tink)

# --------------------------------------------------------

add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../gourd ${CMAKE_CURRENT_BINARY_DIR}/gourd)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/src/*.cpp")

add_executable(loquat ${SOURCE_LIST})
set_target_properties(loquat PROPERTIES LINK_FLAGS_RELEASE -s)

target_link_libraries(loquat gourd
  ${OPENSSL_LIBRARIES}
  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
)

target_include_directories(loquat PUBLIC
  ${PROJECT_SOURCE_DIR}/include
    
  ${spdlog_SOURCE_DIR}/include
  ${argparse_SOURCE_DIR}/include
  ${tomlplusplus_SOURCE_DIR}/include
  ${inja_SOURCE_DIR}/single_include
  ${inja_SOURCE_DIR}/third_party/include
)
