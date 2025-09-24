file(GLOB BEGONIA_SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB GOURD_SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/gourd/src/*")

find_package(argparse CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(SOCI CONFIG REQUIRED)
find_package(unofficial-mariadb-connector-cpp CONFIG REQUIRED)
find_package(redis++ CONFIG REQUIRED)
find_package(unofficial-sodium CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(httplib CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(boost_uuid CONFIG REQUIRED)
find_package(boost_url CONFIG REQUIRED)
find_package(boost_property_tree CONFIG REQUIRED)
find_package(boost_algorithm CONFIG REQUIRED)
find_package(boost_locale CONFIG REQUIRED)
find_package(boost_process CONFIG REQUIRED)
find_package(boost_hana CONFIG REQUIRED)
find_package(jwt-cpp CONFIG REQUIRED)
find_package(mailio CONFIG REQUIRED)
find_package(libgit2 CONFIG REQUIRED)
find_package(PNG REQUIRED)
find_package(Freetype REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(miniocpp CONFIG REQUIRED)
find_package(yaml-cpp CONFIG REQUIRED)
find_package(pugixml CONFIG REQUIRED)
find_package(mstch CONFIG REQUIRED)
find_package(msgpack-cxx CONFIG REQUIRED)
find_package(protobuf CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)
find_package(PkgConfig REQUIRED)

pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

# -----------------------------------------------------------------------------
FetchContent_Declare(thrift
  GIT_REPOSITORY      "https://github.com/apache/thrift.git"
  GIT_TAG             "v0.22.0"
  PATCH_COMMAND       git apply ${PROJECT_SOURCE_DIR}/loquat/patches/thrift.patch
  UPDATE_DISCONNECTED 1
)

FetchContent_Declare(casbin
  GIT_REPOSITORY      "https://github.com/casbin/casbin-cpp.git"
  GIT_TAG             "v1.62.0"
  PATCH_COMMAND       git apply ${PROJECT_SOURCE_DIR}/begonia/patches/casbin.patch
  UPDATE_DISCONNECTED 1
)

FetchContent_Declare(cpp_validator
  GIT_REPOSITORY      "https://github.com/evgeniums/cpp-validator.git"
  GIT_TAG             "c98bbd2"
)

FetchContent_MakeAvailable(thrift casbin cpp_validator)
# -----------------------------------------------------------------------------

add_library(begonia STATIC ${BEGONIA_SOURCE_LIST} ${GOURD_SOURCE_LIST})

target_link_libraries(begonia PUBLIC m dl anl
  argparse::argparse spdlog::spdlog_header_only
  SOCI::soci_core_static SOCI::soci_sqlite3_static SOCI::soci_postgresql_static unofficial::mariadb-connector-cpp::mariadbcpp
  redis++::redis++_static 
  miniocpp::miniocpp
  rabbitmq::rabbitmq-static
  unofficial-sodium::sodium mailio::mailio cpr::cpr jwt-cpp::jwt-cpp
  httplib::httplib nlohmann_json::nlohmann_json pantor::inja
  libgit2::libgit2package PNG::PNG Freetype::Freetype
  PkgConfig::tomlplusplus yaml-cpp::yaml-cpp pugixml::pugixml mstch::mstch msgpack-cxx
  Boost::uuid Boost::url Boost::property_tree Boost::algorithm Boost::locale Boost::process
  protobuf::libprotobuf gRPC::grpc++
  thrift thriftnb thriftz casbin
)

target_include_directories(begonia PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${PROJECT_SOURCE_DIR}/gourd/include

  ${CPPCODEC_INCLUDE_DIRS}

  ${thrift_SOURCE_DIR}/lib/cpp/src
  ${thrift_SOURCE_DIR}/compiler/cpp/src
  ${thrift_BINARY_DIR}
  ${casbin_SOURCE_DIR}/include
  ${cpp_validator_SOURCE_DIR}/include
)
