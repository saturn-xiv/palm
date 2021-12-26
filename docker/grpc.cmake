# https://chromium.googlesource.com/external/github.com/grpc/grpc/+/HEAD/examples/cpp/helloworld/cmake_externalproject/CMakeLists.txt
# https://grpc.io/blog/cmake-improvements/
# https://stackoverflow.com/questions/52202453/cross-compiling-grpc-using-cmake
FetchContent_Declare(
    gRPC
    GIT_REPOSITORY  "https://github.com/grpc/grpc.git"
    GIT_TAG         "v1.43.0"
)
FetchContent_MakeAvailable(gRPC)
