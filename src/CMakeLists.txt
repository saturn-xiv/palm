file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})


# TODO -----------
if(EXISTS ${tink_BINARY_DIR}/include/tink)
else()
  file(INSTALL ${tink_SOURCE_DIR}/cc DESTINATION ${tink_BINARY_DIR}/include)
  file(RENAME ${tink_BINARY_DIR}/include/cc ${tink_BINARY_DIR}/include/tink)
endif()
# ----------------
target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include
  ${tomlplusplus_SOURCE_DIR}
  ${soci_BINARY_DIR}/include
  ${tink_BINARY_DIR}/include
  ${tink_BINARY_DIR}/__generated
)

target_link_libraries(palm PUBLIC
  ${Boost_LIBRARIES}
  
  unofficial::sqlite3::sqlite3
  soci_postgresql_static soci_mysql_static soci_sqlite3_static soci_empty_static soci_core_static
  hiredis::hiredis

  rabbitmq::rabbitmq-static
  libzmq-static

  nlohmann_json::nlohmann_json
  pantor::inja

  Libssh2::libssh2
  zxing::libzxing
  
  ${GIT2_LIBRARY} ${RABBITMQ_LIBRARY} ${MAILIO_LIBRARY}
  
  lemon
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

