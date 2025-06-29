file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

find_package(argparse CONFIG REQUIRED)
find_package(redis++ CONFIG REQUIRED)
find_package(unofficial-sodium CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(libpqxx CONFIG REQUIRED)
find_package(unofficial-libmariadb CONFIG REQUIRED)
find_package(SQLiteCpp CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(httplib CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(boost_uuid CONFIG REQUIRED)
find_package(boost_log_setup CONFIG REQUIRED)
find_package(boost_log CONFIG REQUIRED)
find_package(boost_url CONFIG REQUIRED)
find_package(boost_program_options CONFIG REQUIRED)
find_package(boost_property_tree CONFIG REQUIRED)
find_package(boost_algorithm CONFIG REQUIRED)
find_package(jwt-cpp CONFIG REQUIRED)
find_package(mailio CONFIG REQUIRED)
find_package(PahoMqttCpp CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(miniocpp CONFIG REQUIRED)
find_package(yaml-cpp CONFIG REQUIRED)
find_package(pugixml CONFIG REQUIRED)
find_package(mstch CONFIG REQUIRED)
find_package(PkgConfig REQUIRED)

pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")

add_library(begonia STATIC ${SOURCE_LIST} ${PROTOCOL_SOURCE_LIST})

target_link_libraries(begonia PUBLIC gourd
  argparse::argparse
  libpqxx::pqxx unofficial::libmariadb SQLiteCpp
  redis++::redis++_static 
  miniocpp::miniocpp
  rabbitmq::rabbitmq-static PahoMqttCpp::paho-mqttpp3-static
  unofficial-sodium::sodium mailio cpr::cpr jwt-cpp::jwt-cpp
  httplib::httplib nlohmann_json::nlohmann_json pantor::inja
  PkgConfig::tomlplusplus yaml-cpp::yaml-cpp pugixml::pugixml mstch::mstch
  Boost::log_setup Boost::log Boost::uuid Boost::program_options Boost::url Boost::property_tree Boost::algorithm
)

target_include_directories(begonia PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include

  ${CPPCODEC_INCLUDE_DIRS}
)

source_group(
  TREE "${CMAKE_CURRENT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST} ${PROTOCOL_HEADER_LIST}
)


