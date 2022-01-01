set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

set(target x86_64-linux-gnu)
set(clang_home $ENV{HOME}/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04)

set(CMAKE_C_COMPILER ${clang_home}/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_CXX_COMPILER ${clang_home}/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${target})
set(CMAKE_CXX_FLAGS "-I$clang_home/include/c++/v1 -stdlib=libc++")
set(CMAKE_EXE_LINKER_FLAGS "--rtlib=libgcc --stdlib=libc++ --ld-path=${clang_home}/bin/ld.lld -Wl,-L${clang_home}/lib -rpath $clang_home/lib -Wl,--build-id")
