file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/protocols/cpp/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/protocols/cpp/*.cc")

add_library(lemon STATIC ${SOURCE_LIST})

target_include_directories(lemon PUBLIC
  ${palm_SOURCE_DIR}/protocols/cpp/
)

target_link_libraries(lemon PUBLIC
  dl crypt m ${CMAKE_THREAD_LIBS_INIT} ${CONAN_LIBS}
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/protocols/cpp"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
