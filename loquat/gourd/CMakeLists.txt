file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

add_library(gourd STATIC ${SOURCE_LIST})
target_link_directories(gourd PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/lib)

target_link_libraries(gourd PUBLIC
  event_core event_extra event_pthreads
  thrift thriftnb thriftz
)

target_include_directories(gourd PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/src
)

source_group(
  TREE "${CMAKE_CURRENT_SOURCE_DIR}/src"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
