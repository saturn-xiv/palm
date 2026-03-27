file(GLOB LIB_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*)

find_package(gRPC CONFIG REQUIRED)

add_library(belladonna STATIC ${LIB_SOURCES}) 

target_link_libraries(belladonna PUBLIC
  gRPC::grpc++
)

target_include_directories(belladonna PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include  
)
