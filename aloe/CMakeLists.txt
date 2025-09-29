file(GLOB BAMBOO_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(aloe ${BAMBOO_SOURCES})
target_link_libraries(aloe PRIVATE begonia)
target_link_options(aloe PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(aloe PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(aloe PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
