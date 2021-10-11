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
find_package(Boost REQUIRED COMPONENTS 
    system locale log_setup log program_options
    date_time chrono timer random
    unit_test_framework
)
find_package(OpenSSL REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(SQLite3 REQUIRED)
find_package(CURL REQUIRED)
# --------------------------------------------------------
FetchContent_Declare(
    libpqxx
    GIT_REPOSITORY  "https://github.com/jtv/libpqxx.git"
    GIT_TAG         "7f79dba"
)
FetchContent_MakeAvailable(libpqxx)

FetchContent_Declare(
    SQLiteCpp
    GIT_REPOSITORY  "https://github.com/SRombauts/SQLiteCpp.git"
    GIT_TAG         "3.1.1"
)
FetchContent_MakeAvailable(SQLiteCpp)

FetchContent_Declare(
    hiredis
    GIT_REPOSITORY  "https://github.com/redis/hiredis.git"
    GIT_TAG         "783a378"
)
FetchContent_MakeAvailable(hiredis)

FetchContent_Declare(
    cpp_httplib
    GIT_REPOSITORY  "https://github.com/yhirose/cpp-httplib.git"
    GIT_TAG         "c7554cc"
)
FetchContent_MakeAvailable(cpp_httplib)

FetchContent_Declare(
    jwt_cpp
    GIT_REPOSITORY  "https://github.com/Thalhammer/jwt-cpp.git"
    GIT_TAG         "ab1a60e"
)
FetchContent_MakeAvailable(jwt_cpp)

FetchContent_Declare(
    tomlplusplus
    GIT_REPOSITORY  "https://github.com/marzer/tomlplusplus.git"
    GIT_TAG         "v2.5.0"
)
FetchContent_MakeAvailable(tomlplusplus)

FetchContent_Declare(
    yaml_cpp
    GIT_REPOSITORY  "https://github.com/jbeder/yaml-cpp.git"
    GIT_TAG         "yaml-cpp-0.7.0"
)
FetchContent_MakeAvailable(yaml_cpp)

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
    cppcodec
    GIT_REPOSITORY  "https://github.com/tplgy/cppcodec.git"
    GIT_TAG         "9838f9e"
)
FetchContent_MakeAvailable(cppcodec)

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
#     GIT_TAG         "v3.17.3"
# )
# FetchContent_MakeAvailable(protobuf)

# https://chromium.googlesource.com/external/github.com/grpc/grpc/+/HEAD/examples/cpp/helloworld/cmake_externalproject/CMakeLists.txt
# https://grpc.io/blog/cmake-improvements/
# https://stackoverflow.com/questions/52202453/cross-compiling-grpc-using-cmake
FetchContent_Declare(
    gRPC
    GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
    GIT_TAG         "v1.41.0"
)
FetchContent_MakeAvailable(gRPC)

# FetchContent_Declare(
#     tink
#     GIT_REPOSITORY  "https://github.com/google/tink.git"
#     GIT_TAG         "v1.6.1"
# )
# FetchContent_MakeAvailable(tink)

# https://github.com/google/tink/blob/master/docs/CMAKE-HOWTO.md
# add_subdirectory(vendors/tink)

FetchContent_Declare(
    cpr
    GIT_REPOSITORY  "https://github.com/libcpr/cpr.git"
    GIT_TAG         "1.6.2"
)
FetchContent_MakeAvailable(cpr)
