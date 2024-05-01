cmake_minimum_required(VERSION 3.22)
include(ExternalProject)
include(FetchContent)

set(CMAKE_TOOLCHAIN_FILE "${CMAKE_CURRENT_SOURCE_DIR}/vcpkg/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")

project(
    loquat
    VERSION 2024.5.1
    DESCRIPTION "A cryptographic rpc service(by google tink & casbin)."
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
find_package(spdlog CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(libpqxx CONFIG REQUIRED)
find_package(unofficial-sqlite3 CONFIG REQUIRED)
find_package(SQLiteCpp CONFIG REQUIRED)
find_package(leveldb CONFIG REQUIRED)
find_package(RdKafka CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(eclipse-paho-mqtt-c CONFIG REQUIRED)
find_package(PahoMqttCpp CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)
find_package(protobuf CONFIG REQUIRED)
find_package(boost_uuid CONFIG REQUIRED)

find_package(PkgConfig REQUIRED)
pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS ")cppcodec/base32_crockford.hpp")

# --------------------------------------------------------

# https://github.com/tink-crypto/tink-cc
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.1.3"
)
# https://github.com/casbin/casbin-cpp
FetchContent_Declare(casbin
  GIT_REPOSITORY  "https://github.com/casbin/casbin-cpp.git"
  GIT_TAG         "v1.59.0"
)

FetchContent_MakeAvailable(tink casbin)

# --------------------------------------------------------

add_subdirectory(gourd)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/src/*.cpp")

add_executable(loquat ${SOURCE_LIST})
set_target_properties(loquat PROPERTIES LINK_FLAGS_RELEASE -s)

target_link_libraries(loquat gourd
  OpenSSL::SSL OpenSSL::Crypto
  spdlog::spdlog_header_only
  Boost::uuid PkgConfig::tomlplusplus nlohmann_json::nlohmann_json pantor::inja
  leveldb::leveldb unofficial::sqlite3::sqlite3 SQLiteCpp PostgreSQL::PostgreSQL libpqxx::pqxx
  rabbitmq::rabbitmq-static RdKafka::rdkafka RdKafka::rdkafka++ eclipse-paho-mqtt-c::paho-mqtt3c-static PahoMqttC::PahoMqttC PahoMqttCpp::paho-mqttpp3-static
  protobuf::libprotoc protobuf::libprotobuf gRPC::gpr gRPC::grpc gRPC::grpc++ gRPC::grpc++_alts

  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
)

target_include_directories(loquat PUBLIC
  ${PROJECT_SOURCE_DIR}/include

  ${CPPCODEC_INCLUDE_DIRS}
)


