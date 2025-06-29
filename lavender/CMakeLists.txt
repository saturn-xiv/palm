file(GLOB LAVENDER_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(lavender ${LAVENDER_SOURCES})
target_link_libraries(lavender PRIVATE begonia)
target_link_options(lavender PRIVATE -static-libgcc -static-libstdc++)
set_target_properties(lavender PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(bamboo PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
