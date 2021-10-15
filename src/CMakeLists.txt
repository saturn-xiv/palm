file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})

# ----------------
target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include

  ${CPPCODEC_INCLUDE_DIRS}
  ${JWT_CPP_INCLUDE_DIRS}
  ${CPP_HTTPLIB_INCLUDE_DIRS}

  ${tomlplusplus_SOURCE_DIR}
  ${cppcodec_SOURCE_DIR}
  ${cpp_httplib_SOURCE_DIR}
  ${hiredis_SOURCE_DIR}
  ${jwt_cpp_SOURCE_DIR}/include
  ${inja_SOURCE_DIR}/include
  ${inja_SOURCE_DIR}/third_party/include
  ${rabbitmq_SOURCE_DIR}/librabbitmq
  ${libzmq_SOURCE_DIR}/include
  ${cpr_SOURCE_DIR}/include
)

target_link_libraries(palm PUBLIC lemon
  ${Boost_LIBRARIES} 
  OpenSSL::SSL OpenSSL::Crypto
  CURL::libcurl
  hiredis::hiredis
  libzmq-static cppzmq-static
  PostgreSQL::PostgreSQL
  libmariadb mariadbclient
  unofficial::sqlite3::sqlite3 SQLiteCpp
  tomlplusplus::tomlplusplus
  nlohmann_json::nlohmann_json
  yaml-cpp
  tinyxml2::tinyxml2
  pantor::inja
  rabbitmq::rabbitmq-static
  eclipse-paho-mqtt-c::paho-mqtt3c-static
  flatbuffers::flatbuffers
  
  pqxx
  libserial_static
  cpr::cpr
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

