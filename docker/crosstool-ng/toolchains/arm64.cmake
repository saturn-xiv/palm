set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)

set(target aarch64-unknown-linux-gnu)
set(clang_home $ENV{HOME}/local/clang+llvm-13.0.0-x86_64-linux-gnu-ubuntu-20.04)
set(gcc_home $ENV{HOME}/x-tools/${target})

set(CMAKE_C_COMPILER ${clang_home}/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_CXX_COMPILER ${clang_home}/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${target})
set(CMAKE_C_FLAGS "--sysroot ${gcc_home}/${target}/sysroot -ccc-gcc-name ${gcc_home}/bin/${target}-gcc")
set(CMAKE_CXX_FLAGS "--sysroot ${gcc_home}/${target}/sysroot -ccc-gcc-name ${gcc_home}/bin/${target}-gcc")
set(CMAKE_EXE_LINKER_FLAGS "-stdlib=libstdc++ -gcc-toolchain ${gcc_home} --sysroot ${gcc_home}/${target}/sysroot --ld-path=${clang_home}/bin/ld.lld -Wl,--build-id")

set(CMAKE_FIND_ROOT_PATH ${gcc_home}/${target}/sysroot)
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
