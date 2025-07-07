set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR riscv64)

set(target riscv64-linux-gnu)


# set(CMAKE_C_COMPILER clang)
# set(CMAKE_C_COMPILER_TARGET ${target})
# set(CMAKE_C_FLAGS "-ccc-gcc-name ${target}-gcc")
# set(CMAKE_CXX_COMPILER clang++)
# set(CMAKE_CXX_COMPILER_TARGET ${target})
# set(CMAKE_CXX_FLAGS "-stdlib=libstdc++ -ccc-gcc-name ${target}-gcc -fclang-abi-compat=17")


set(CMAKE_C_COMPILER ${target}-gcc)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_CXX_COMPILER ${target}-g++)
set(CMAKE_CXX_COMPILER_TARGET ${target})


# set(CMAKE_EXE_LINKER_FLAGS "-fuse-ld=mold -s -Wl,--build-id=sha1")

