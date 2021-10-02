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
