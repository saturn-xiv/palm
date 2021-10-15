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
    date_time chrono timer random pool
    unit_test_framework
)
find_package(OpenSSL REQUIRED)
find_package(CURL REQUIRED)
find_package(hiredis CONFIG REQUIRED)
find_package(ZeroMQ CONFIG REQUIRED)
find_package(cppzmq CONFIG REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(unofficial-libmariadb CONFIG REQUIRED)
find_package(unofficial-sqlite3 CONFIG REQUIRED)
find_package(SQLiteCpp CONFIG REQUIRED)
find_package(tomlplusplus CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(yaml-cpp CONFIG REQUIRED)
find_package(tinyxml2 CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(eclipse-paho-mqtt-c CONFIG REQUIRED)
find_package(Flatbuffers CONFIG REQUIRED)
    
find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")
find_path(JWT_CPP_INCLUDE_DIRS "jwt-cpp/base.h")
find_path(CPP_HTTPLIB_INCLUDE_DIRS "httplib.h")
        
# --------------------------------------------------------

FetchContent_Declare(
    tomlplusplus
    GIT_REPOSITORY  "https://github.com/marzer/tomlplusplus.git"
    GIT_TAG         "v2.5.0"
)
FetchContent_MakeAvailable(tomlplusplus)

FetchContent_Declare(
    libpqxx
    GIT_REPOSITORY  "https://github.com/jtv/libpqxx.git"
    GIT_TAG         "7f79dba"
)
FetchContent_MakeAvailable(libpqxx)


FetchContent_Declare(
    libserial
    GIT_REPOSITORY  "https://github.com/crayzeewulf/libserial.git"
    GIT_TAG         "1d1e47a"
)
FetchContent_MakeAvailable(libserial)


# https://chromium.googlesource.com/external/github.com/grpc/grpc/+/HEAD/examples/cpp/helloworld/cmake_externalproject/CMakeLists.txt
# https://grpc.io/blog/cmake-improvements/
# https://stackoverflow.com/questions/52202453/cross-compiling-grpc-using-cmake
FetchContent_Declare(
    gRPC
    GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
    GIT_TAG         "v1.41.0"
)
FetchContent_MakeAvailable(gRPC)

FetchContent_Declare(
    cpr
    GIT_REPOSITORY  "https://github.com/libcpr/cpr.git"
    GIT_TAG         "1.6.2"
)
FetchContent_MakeAvailable(cpr)
