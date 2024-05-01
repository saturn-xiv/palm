cmake_minimum_required(VERSION 3.22)
include(ExternalProject)
include(FetchContent)
include(GNUInstallDirs)

project(
    loquat
    VERSION 2024.5.2
    DESCRIPTION "A cryptographic dynamic library(wraped google tink)."
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
configure_file(include/version.h.in ${PROJECT_SOURCE_DIR}/include/version.hpp)

# --------------------------------------------------------

find_package(OpenSSL REQUIRED)

# --------------------------------------------------------

# https://github.com/tink-crypto/tink-cc
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.1.3"
)

# https://github.com/gabime/spdlog
FetchContent_Declare(spdlog
  GIT_REPOSITORY  "https://github.com/gabime/spdlog.git"
  GIT_TAG         "v1.14.1"
)

FetchContent_MakeAvailable(spdlog tink)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/src/*.cpp")

add_library(gourd ${SOURCE_LIST})
set_target_properties(gourd PROPERTIES LINK_FLAGS_RELEASE -s)
set_target_properties(gourd PROPERTIES VERSION ${PROJECT_VERSION})

target_link_libraries(gourd
  ${OPENSSL_LIBRARIES}
  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
)

target_include_directories(gourd PUBLIC
  ${PROJECT_SOURCE_DIR}/include

  ${spdlog_SOURCE_DIR}/include
)


install(TARGETS gourd
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}
)
