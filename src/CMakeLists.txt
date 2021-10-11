file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})

# ----------------
target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include
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
  ${Boost_LIBRARIES} ${PostgreSQL_LIBRARIES} ${SQLite3_LIBRARIES} ${CURL_LIBRARIES}

  pqxx SQLiteCpp hiredis_static
  rabbitmq-static libzmq-static
  flatbuffers
  libserial_static yaml-cpp cpr::cpr
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

