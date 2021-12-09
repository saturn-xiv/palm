file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/models/include/palm/include/palm/models/*.h")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/models/src/*.cpp")

add_library(peony STATIC ${SOURCE_LIST})

target_include_directories(peony PUBLIC
  ${palm_SOURCE_DIR}/models/include/
)

target_link_libraries(peony PUBLIC ${CONAN_LIBS})

source_group(
  TREE "${PROJECT_SOURCE_DIR}/models/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)
