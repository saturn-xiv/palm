file(GLOB BEGONIA_SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB GOURD_SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/gourd/src/*")

# -----------------------------------------------------------------------------
FetchContent_Declare(libpqxx
  GIT_REPOSITORY "https://github.com/jtv/libpqxx.git"
  GIT_TAG        "7.10.4"
)

FetchContent_Declare(absl
  GIT_REPOSITORY "https://github.com/abseil/abseil-cpp.git"
  GIT_TAG        "20250814.1"
)

FetchContent_Declare(gRPC
  GIT_REPOSITORY "https://github.com/grpc/grpc.git"
  GIT_TAG        "v1.76.0"
)

FetchContent_MakeAvailable(absl libpqxx gRPC)
# -----------------------------------------------------------------------------

add_library(begonia STATIC ${BEGONIA_SOURCE_LIST} ${GOURD_SOURCE_LIST})

target_link_libraries(begonia PUBLIC m dl anl
  grpc++ pqxx
)

target_include_directories(begonia PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${PROJECT_SOURCE_DIR}/gourd/include
)
