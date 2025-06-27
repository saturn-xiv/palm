cmake_minimum_required(VERSION 3.28)
include(ExternalProject)
include(FetchContent)

project(
    loquat
    VERSION 2025.6.27
    DESCRIPTION "A cryptographic rpc service(by Google Tink)."
    LANGUAGES C CXX)
# --------------------------------------------------------

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
configure_file(include/loquat/version.h.in ${PROJECT_SOURCE_DIR}/include/loquat/version.hpp)

# --------------------------------------------------------

FetchContent_Declare(spdlog
  GIT_REPOSITORY  "https://github.com/gabime/spdlog.git"
  GIT_TAG         "v1.15.3"
)
FetchContent_Declare(tink
  GIT_REPOSITORY  "https://github.com/tink-crypto/tink-cc.git"
  GIT_TAG         "v2.4.0"
)


FetchContent_MakeAvailable(spdlog tink)

# --------------------------------------------------------

file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/src/*.cpp" "${PROJECT_SOURCE_DIR}/src/*.cc")

add_library(loquat SHARED ${SOURCE_LIST})
set_property(TARGET loquat PROPERTY POSITION_INDEPENDENT_CODE ON)
set_target_properties(loquat PROPERTIES LINK_FLAGS_RELEASE -s)

target_link_libraries(loquat
  tink::static tink::jwt::jwt_key_templates tink::jwt::jwt_mac tink::jwt::jwt_mac_config
)

target_include_directories(loquat PUBLIC
  ${PROJECT_SOURCE_DIR}/include
    
  ${spdlog_SOURCE_DIR}/include
)
