cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
    loquat
    VERSION 2026.7.23
    DESCRIPTION "A cryptographic rpc service(by Google Tink)."
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

# --------------------------------------------------------

# https://github.com/tink-crypto/tink-cc
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.8.0"
)

FetchContent_MakeAvailable(tink)

# --------------------------------------------------------

add_subdirectory(gourd)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/src/*.cpp")

add_executable(loquat ${SOURCE_LIST})
set_target_properties(loquat PROPERTIES LINK_FLAGS_RELEASE -s)
target_link_options(loquat PRIVATE -static-libgcc -static-libstdc++)

target_link_libraries(loquat gourd
  # argparse::argparse
  # spdlog::spdlog_header_only
  # pantor::inja
 

  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
  # thriftz thriftnb
)

target_include_directories(loquat PUBLIC
  ${PROJECT_SOURCE_DIR}/include
)


