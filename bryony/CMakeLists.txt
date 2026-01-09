cmake_minimum_required(VERSION 3.22)
include(FetchContent)

project(
  bryony
  VERSION 2026.1.9
  DESCRIPTION "A printing service by imap/smtp."
  LANGUAGES CXX
)

# ---------------------------------------------------------

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# --------------------------------------------------------

execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(include/bryony/version.h.in ${CMAKE_CURRENT_SOURCE_DIR}/include/bryony/version.hpp)

# ---------------------------------------------------------
FetchContent_Declare(argparse
  GIT_REPOSITORY "https://github.com/p-ranav/argparse.git"
  GIT_TAG        "v3.2"
)

FetchContent_Declare(spdlog
  GIT_REPOSITORY "https://github.com/gabime/spdlog.git"
  GIT_TAG        "v1.16.0"
)

FetchContent_Declare(tomlplusplus
  GIT_REPOSITORY "https://github.com/marzer/tomlplusplus.git"
  GIT_TAG        "v3.4.0"
)

FetchContent_Declare(cppcodec
  GIT_REPOSITORY "https://github.com/tplgy/cppcodec.git"
  GIT_TAG        "8019b8b"
)


FetchContent_Declare(stduuid
  GIT_REPOSITORY "https://github.com/mariusbancila/stduuid.git"
  GIT_TAG        "3afe719"
)

FetchContent_Declare(inja
  GIT_REPOSITORY "https://github.com/pantor/inja.git"
  GIT_TAG        "v3.5.0"
)

FetchContent_Declare(mailio
  GIT_REPOSITORY "https://github.com/karastojko/mailio.git"
  GIT_TAG        "af96e87"
)

FetchContent_Declare(librabbitmq
  GIT_REPOSITORY "https://github.com/alanxz/rabbitmq-c.git"
  GIT_TAG        "8b7471e"
)

FetchContent_MakeAvailable(argparse spdlog tomlplusplus cppcodec stduuid inja mailio librabbitmq)
# ---------------------------------------------------------
file(GLOB BRYONY_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(bryony ${BRYONY_SOURCES})
target_link_options(bryony PRIVATE -Wl,--build-id=sha1)
set_target_properties(bryony PROPERTIES LINK_FLAGS_RELEASE -s)
target_link_libraries(bryony PRIVATE m dl cups
  argparse tomlplusplus::tomlplusplus mailio::mailio rabbitmq-static
)   

target_include_directories(bryony PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include

  ${cppcodec_SOURCE_DIR}
  ${spdlog_SOURCE_DIR}/include
  ${tomlplusplus_SOURCE_DIR}/include
  ${stduuid_SOURCE_DIR}/include
  ${inja_SOURCE_DIR}/single_include
  ${inja_SOURCE_DIR}/third_party/include
  ${mailio_SOURCE_DIR}/include
  ${librabbitmq_SOURCE_DIR}/include
)
