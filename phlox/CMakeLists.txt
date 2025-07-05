file(GLOB PHLOX_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(phlox ${PHLOX_SOURCES})
target_link_libraries(phlox PRIVATE begonia)
target_link_options(phlox PRIVATE -static-libgcc -static-libstdc++)
set_target_properties(phlox PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(phlox PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
