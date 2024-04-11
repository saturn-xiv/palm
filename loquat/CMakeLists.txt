cmake_minimum_required(VERSION 3.24)
include(ExternalProject)
include(FetchContent)

project(
    loquat
    VERSION 2024.4.11
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

# https://github.com/gabime/spdlog
FetchContent_Declare(spdlog
  GIT_REPOSITORY  "https://github.com/gabime/spdlog.git"
  GIT_TAG         "v1.13.0"
)
# https://github.com/marzer/tomlplusplus
FetchContent_Declare(tomlplusplus
  GIT_REPOSITORY  "https://github.com/marzer/tomlplusplus.git"
  GIT_TAG         "v3.4.0"
)
# https://github.com/p-ranav/argparse
FetchContent_Declare(argparse
  GIT_REPOSITORY  "https://github.com/p-ranav/argparse.git"
  GIT_TAG         "v3.0"
)
# https://github.com/tink-crypto/tink-cc
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "ac71903"
)
# https://github.com/casbin/casbin-cpp
FetchContent_Declare(casbin
  GIT_REPOSITORY  "https://github.com/casbin/casbin-cpp.git"
  GIT_TAG         "v1.55.0"
)
# https://github.com/google/leveldb
FetchContent_Declare(leveldb
  GIT_REPOSITORY  "https://github.com/google/leveldb.git"
  GIT_TAG         "1.23"
)
# https://github.com/alanxz/rabbitmq-c
FetchContent_Declare(rabbitmq
  GIT_REPOSITORY  "https://github.com/alanxz/rabbitmq-c.git"
  GIT_TAG         "v0.14.0"
)
# https://github.com/confluentinc/librdkafka
FetchContent_Declare(librdkafka
  GIT_REPOSITORY  "https://github.com/confluentinc/librdkafka.git"
  GIT_TAG         "v2.3.0"
)

FetchContent_MakeAvailable(spdlog argparse tomlplusplus tink casbin leveldb rabbitmq librdkafka)

# --------------------------------------------------------

add_subdirectory(gourd)

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
)


