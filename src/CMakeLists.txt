file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})

# ----------------
target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include
  ${tomlplusplus_SOURCE_DIR}
)

target_link_libraries(palm PUBLIC
  Poco::Crypto Poco::JWT Poco::JSON Poco::Zip Poco::XML Poco::Util
  Poco::DataPostgreSQL Poco::DataMySQL Poco::DataSQLite
  Poco::Redis Poco::MongoDB

  rabbitmq-static libzmq-static
  flatbuffers
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

