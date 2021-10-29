set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR armv7l)

set(target arm-linux-gnueabihf)

set(CMAKE_C_COMPILER clang-13)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_CXX_COMPILER clang++-13)
set(CMAKE_CXX_COMPILER_TARGET ${target})
set(CMAKE_C_FLAGS "-ccc-gcc-name ${target}-gcc-10")
set(CMAKE_CXX_FLAGS "-stdlib=libstdc++ -ccc-gcc-name ${target}-gcc-10")
set(CMAKE_EXE_LINKER_FLAGS "--ld-path=ld.lld-13")

set(CMAKE_FIND_ROOT_PATH /usr/${target})
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
