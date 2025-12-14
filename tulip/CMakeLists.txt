file(GLOB TULIP_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(tulip ${TULIP_SOURCES})
target_link_libraries(tulip PRIVATE begonia)
target_link_options(tulip PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(tulip PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(tulip PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
