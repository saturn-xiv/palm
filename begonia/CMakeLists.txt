file(GLOB BEGONIA_SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB GOURD_SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/gourd/src/*")

# -----------------------------------------------------------------------------
FetchContent_Declare(gRPC
  GIT_REPOSITORY "https://github.com/grpc/grpc.git"
  GIT_TAG        "v1.76.0"
)

FetchContent_MakeAvailable(gRPC)
# -----------------------------------------------------------------------------

add_library(begonia STATIC ${BEGONIA_SOURCE_LIST} ${GOURD_SOURCE_LIST})

target_link_libraries(begonia PUBLIC m dl anl
  gRPC::grpc++
)

target_include_directories(begonia PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${PROJECT_SOURCE_DIR}/gourd/include
)
