set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)

set(target aarch64-unknown-linux-gnu)

set(CMAKE_C_COMPILER $ENV{HOME}/x-tools/${target}/bin/${target}-gcc)
set(CMAKE_C_COMPILER_TARGET ${target})
set(CMAKE_CXX_COMPILER $ENV{HOME}/x-tools/${target}/bin/${target}-g++)
set(CMAKE_CXX_COMPILER_TARGET ${target})

set(BOOST_CHARCONV_QUADMATH_FOUND_EXITCODE 0)
