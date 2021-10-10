file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})

# ----------------
target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include
  ${tomlplusplus_SOURCE_DIR}
  ${cppcodec_SOURCE_DIR}
  ${inja_SOURCE_DIR}/include
  ${inja_SOURCE_DIR}/third_party/include
)

target_link_libraries(palm PUBLIC
  ${Boost_LIBRARIES} ${PostgreSQL_LIBRARIES} ${SQLite3_LIBRARIES}

  pqxx SQLiteCpp hiredis_static
  rabbitmq-static libzmq-static
  flatbuffers
  libserial_static yaml-cpp
  # tink::static

  dl crypt m ${CMAKE_THREAD_LIBS_INIT} ${OPENSSL_LIBRARIES}
  grpc grpc++ grpc++_reflection
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

