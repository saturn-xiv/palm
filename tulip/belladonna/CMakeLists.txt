file(GLOB LIB_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*)

find_package(gRPC CONFIG REQUIRED)

make_directory("${VCPKG_INSTALLED_DIR}/${VCPKG_TARGET_TRIPLET}/bin")
find_package(Thrift CONFIG REQUIRED)

add_library(belladonna STATIC ${LIB_SOURCES}) 

target_link_libraries(belladonna PUBLIC
  gRPC::grpc++
  thrift::thrift thriftz::thriftz
)

target_include_directories(belladonna PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include  
)
