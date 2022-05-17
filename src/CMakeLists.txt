file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})

target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include
)

target_link_libraries(palm PUBLIC dl crypt m
  rabbitmq
  Boost::locale Boost::log Boost::log_setup
  Boost::chrono Boost::date_time Boost::timer Boost::random Boost::system Boost::filesystem
  Poco::Crypto Poco::JWT Poco::XML Poco::JSON Poco::Zip
  Poco::Data Poco::DataSQLite Poco::DataMySQL Poco::DataPostgreSQL Poco::MongoDB Poco::Redis
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

