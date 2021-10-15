set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR armv7l)

set(triple arm-linux-gnueabihf)

set(CMAKE_C_COMPILER clang-13)
set(CMAKE_C_COMPILER_TARGET ${triple})
set(CMAKE_CXX_COMPILER clang++-13)
set(CMAKE_CXX_COMPILER_TARGET ${triple})
