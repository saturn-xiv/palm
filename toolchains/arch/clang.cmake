set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

set(target x86_64-linux-gnu)

set(CMAKE_C_COMPILER /usr/lib/llvm19/bin/clang)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_CXX_COMPILER /usr/lib/llvm19/bin/clang++)
set(CMAKE_CXX_COMPILER_TARGET ${target})
# https://github.com/abseil/abseil-cpp/issues/1747#issuecomment-2308667626
set(CMAKE_CXX_FLAGS "-stdlib=libstdc++ -fclang-abi-compat=17")

set(CMAKE_EXE_LINKER_FLAGS "-fuse-ld=mold -s -Wl,--build-id=sha1")
