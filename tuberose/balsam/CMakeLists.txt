file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/include/balsam/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

add_executable(balsam ${SOURCE_LIST})
set_target_properties(balsam PROPERTIES LINK_FLAGS_RELEASE -s)

find_package(cppzmq CONFIG REQUIRED)
find_package(PahoMqttCpp CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(miniocpp CONFIG REQUIRED)

pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

FetchContent_Declare(casbin
  GIT_REPOSITORY    "https://github.com/casbin/casbin-cpp.git"
  GIT_TAG           "v1.59.0"
  PATCH_COMMAND     git apply "${CMAKE_CURRENT_SOURCE_DIR}/casbin.patch"
)
FetchContent_MakeAvailable(casbin)

target_link_libraries(balsam PUBLIC 
  lemon gourd casbin
 
  cppzmq-static PahoMqttCpp::paho-mqttpp3-static rabbitmq::rabbitmq-static
  
  cpr::cpr miniocpp::miniocpp
  
  PkgConfig::tomlplusplus

  spdlog::spdlog_header_only
)

target_include_directories(balsam PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${PROJECT_SOURCE_DIR}/include

  ${CPPCODEC_INCLUDE_DIRS}  
)

source_group(
  TREE "${CMAKE_CURRENT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

