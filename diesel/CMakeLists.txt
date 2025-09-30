file(GLOB DIESEL_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(diesel ${DIESEL_SOURCES})
target_link_libraries(diesel PRIVATE begonia)
target_link_options(diesel PRIVATE -static-libgcc -static-libstdc++ -Wl,--build-id=sha1)
set_target_properties(diesel PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(diesel PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
