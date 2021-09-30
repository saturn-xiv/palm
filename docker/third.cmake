# --------------------------------------------------------
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)
# --------------------------------------------------------
execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
# --------------------------------------------------------
find_package(Threads REQUIRED)
find_package(OpenSSL REQUIRED)
find_package(PostgreSQL REQUIRED)
# --------------------------------------------------------
FetchContent_Declare(poco
  GIT_REPOSITORY    "https://github.com/pocoproject/poco.git"
  GIT_TAG           "poco-1.11.0-release"
)
FetchContent_MakeAvailable(poco)

FetchContent_Declare(
    tomlplusplus
    GIT_REPOSITORY  "https://github.com/marzer/tomlplusplus.git"
    GIT_TAG         "v2.5.0"
)
FetchContent_MakeAvailable(tomlplusplus)

FetchContent_Declare(
    libzmq
    GIT_REPOSITORY  "https://github.com/zeromq/libzmq.git"
    GIT_TAG         "v4.3.4"
)
FetchContent_MakeAvailable(libzmq)

FetchContent_Declare(
    rabbitmq
    GIT_REPOSITORY  "https://github.com/alanxz/rabbitmq-c.git"
    GIT_TAG         "v0.11.0"
)
FetchContent_MakeAvailable(rabbitmq)

FetchContent_Declare(
    inja
    GIT_REPOSITORY  "https://github.com/pantor/inja.git"
    GIT_TAG         "v3.3.0"
)
FetchContent_MakeAvailable(inja)

FetchContent_Declare(
    libserial
    GIT_REPOSITORY  "https://github.com/crayzeewulf/libserial.git"
    GIT_TAG         "1d1e47a"
)
FetchContent_MakeAvailable(libserial)

FetchContent_Declare(
    flatbuffers
    GIT_REPOSITORY  "https://github.com/google/flatbuffers.git"
    GIT_TAG         "b9d43a5"
)
FetchContent_MakeAvailable(flatbuffers)

# https://github.com/protocolbuffers/protobuf
# FetchContent_Declare(
#     protobuf
#     GIT_REPOSITORY  "https://github.com/protocolbuffers/protobuf.git"
#     GIT_TAG         "v3.15.8"
# )
# set(protobuf_BUILD_TESTS OFF)

# https://chromium.googlesource.com/external/github.com/grpc/grpc/+/HEAD/examples/cpp/helloworld/cmake_externalproject/CMakeLists.txt
# https://grpc.io/blog/cmake-improvements/
# https://stackoverflow.com/questions/52202453/cross-compiling-grpc-using-cmake
FetchContent_Declare(
    gRPC
    GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
    GIT_TAG         "v1.41.0"
)
FetchContent_MakeAvailable(gRPC)
# set(Protobuf_PROTOC_EXECUTABLE $ENV{HOME}/local/vcpkg/installed/x64-linux/tools/protobuf/protoc)


# --------------------------------------------------------

# find_package(ZLIB REQUIRED)
# find_package(hiredis CONFIG REQUIRED)
# find_package(rabbitmq-c CONFIG REQUIRED)
# find_package(nlohmann_json CONFIG REQUIRED)
# find_package(inja CONFIG REQUIRED)
# find_package(Libssh2 CONFIG REQUIRED)
# find_package(unofficial-sqlite3 CONFIG REQUIRED)
# find_package(Boost REQUIRED COMPONENTS 
#     system locale log_setup log thread program_options
#     date_time chrono timer random
#     unit_test_framework
# )
# find_package(bigint CONFIG REQUIRED)
# find_package(zxing CONFIG REQUIRED)
# find_package(absl CONFIG REQUIRED)
# find_package(ZeroMQ CONFIG REQUIRED)
# find_package(protobuf CONFIG REQUIRED)

# find_library(GIT2_LIBRARY git2)
# find_library(RABBITMQ_LIBRARY rabbitmq)
# find_library(MAILIO_LIBRARY mailio)

# --------------------------------------------------------

# set(BUILD_STATIC_LIBS ON)
# set(BUILD_SHARED_LIBS OFF)
# set(BUILD_TESTING OFF)
# set(BUILD_SHARED OFF)
# set(BUILD_STATIC ON)
# set(BUILD_TESTS OFF)

# # http://soci.sourceforge.net/doc/release/4.0/installation/
# # https://github.com/SOCI/soci/pull/884
# FetchContent_Declare(
#     soci
#     GIT_REPOSITORY  "https://github.com/SOCI/soci.git"
#     GIT_TAG         "1a97e58"
# )
# set(SOCI_SHARED OFF)
# set(SOCI_TESTS OFF)
# set(WITH_BOOST ON)
# set(WITH_SQLITE3 ON)
# set(WITH_MYSQL ON)
# set(WITH_POSTGRESQL ON)
# set(WITH_ODBC OFF)
# set(WITH_ORACLE OFF)
# set(WITH_FIREBIRD OFF)
# set(WITH_DB2 OFF)

# # https://github.com/marzer/tomlplusplus


# https://github.com/zeromq/libzmq



# # https://github.com/google/tink
# FetchContent_Declare(
#     tink
#     GIT_REPOSITORY  "https://github.com/google/tink.git"
#     GIT_TAG         "v1.6.1"
# )
# FetchContent_MakeAvailable(gRPC tink tomlplusplus soci)

# --------------------------------------------------------

# link_libraries(stdc++fs dl crypt m
#     

#      ${Boost_LIBRARIES}
#     unofficial::sqlite3::sqlite3
#     soci_postgresql_static soci_mysql_static soci_sqlite3_static soci_empty_static soci_core_static
#     hiredis::hiredis

#     rabbitmq::rabbitmq-static
#     libzmq-static

#     nlohmann_json::nlohmann_json
#     pantor::inja

#     Libssh2::libssh2
    
#     ${GIT2_LIBRARY} ${RABBITMQ_LIBRARY}

#     grpc grpc++ grpc++_reflection
# )

# include_directories(
#     ${tomlplusplus_SOURCE_DIR}
#     ${soci_BINARY_DIR}/include
# )

# --------------------------------------------------------

# TODO PCH
# https://cmake.org/cmake/help/latest/command/target_precompile_headers.html
