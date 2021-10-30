set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR armv7l)

set(target arm-none-linux-gnueabihf)
set(clang_home $ENV{HOME}/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04)
set(gcc_home $ENV{HOME}/local/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf)
set(cflags "-ccc-gcc-name $gcc_home/bin/$target-gcc --gcc-toolchain=$gcc_home")

set(CMAKE_C_COMPILER ${clang_home}/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_C_FLAGS ${cflags})

set(CMAKE_CXX_COMPILER ${clang_home}/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${target})
set(CMAKE_CXX_FLAGS "-stdlib=libstdc++ ${cflags}")

set(CMAKE_EXE_LINKER_FLAGS "--ld-path=${clang_home}/bin/ld.lld")

set(CMAKE_FIND_ROOT_PATH ${gcc_home}/${target}/libc)
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
