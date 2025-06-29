file(GLOB BEGONIA_SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB GOURD_SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/gourd/src/*")

find_package(argparse CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(SOCI CONFIG REQUIRED)
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
find_package(jwt-cpp CONFIG REQUIRED)
find_package(mailio CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(miniocpp CONFIG REQUIRED)
find_package(yaml-cpp CONFIG REQUIRED)
find_package(pugixml CONFIG REQUIRED)
find_package(mstch CONFIG REQUIRED)
find_package(PkgConfig REQUIRED)

pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

add_library(begonia STATIC ${BEGONIA_SOURCE_LIST} ${GOURD_SOURCE_LIST})

target_link_libraries(begonia PUBLIC gourd
  argparse::argparse spdlog::spdlog_header_only
  SOCI::soci_core_static SOCI::soci_sqlite3_static SOCI::soci_postgresql_static
  redis++::redis++_static 
  miniocpp::miniocpp
  rabbitmq::rabbitmq-static
  unofficial-sodium::sodium mailio cpr::cpr jwt-cpp::jwt-cpp
  httplib::httplib nlohmann_json::nlohmann_json pantor::inja
  PkgConfig::tomlplusplus yaml-cpp::yaml-cpp pugixml::pugixml mstch::mstch
  Boost::uuid Boost::url Boost::property_tree Boost::algorithm
)

target_include_directories(begonia PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${PROJECT_SOURCE_DIR}/gourd/include

  ${CPPCODEC_INCLUDE_DIRS}
)
