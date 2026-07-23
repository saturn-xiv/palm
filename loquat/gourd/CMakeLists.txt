file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

add_library(gourd STATIC ${SOURCE_LIST})

target_link_libraries(gourd PUBLIC
  OpenSSL::SSL OpenSSL::Crypto
  libevent::core libevent::extra libevent::openssl libevent::pthreads
  thrift::thrift thriftz::thriftz thriftnb::thriftnb 
)

target_include_directories(gourd PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)

source_group(
  TREE "${CMAKE_CURRENT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
