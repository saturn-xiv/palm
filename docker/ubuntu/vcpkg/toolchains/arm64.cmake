set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)

set(triple aarch64-linux-gnu)

set(CLANG_HOME $ENV{HOME}/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04)
set(GCC_HOME $ENV{HOME}/local/gcc-arm-10.3-2021.07-x86_64-aarch64-none-linux-gnu)

set(CMAKE_C_COMPILER ${CLANG_HOME}/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${triple})
set(CMAKE_CXX_COMPILER ${CLANG_HOME}/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${triple})
set(CMAKE_C_FLAGS "-ccc-gcc-name ${GCC_HOME}/bin/aarch64-none-linux-gnu-gcc")
set(CMAKE_CXX_FLAGS "-stdlib=libstdc++")
set(CMAKE_EXE_LINKER_FLAGS "--ld-path=${CLANG_HOME}/bin/ld.lld")

set(CMAKE_FIND_ROOT_PATH /usr/${triple})
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
