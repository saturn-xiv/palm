set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

set(triple x86_64-linux-gnu)

set(CLANG_HOME $ENV{HOME}/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04)

set(CMAKE_C_COMPILER ${CLANG_HOME}/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${triple})
set(CMAKE_CXX_COMPILER ${CLANG_HOME}/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${triple})
set(CMAKE_CXX_FLAGS "-stdlib=libstdc++")
set(CMAKE_EXE_LINKER_FLAGS "--ld-path=${CLANG_HOME}/bin/ld.lld")
