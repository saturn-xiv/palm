file(GLOB BAMBOO_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(bamboo ${BAMBOO_SOURCES})
target_link_libraries(bamboo PRIVATE begonia)
target_link_options(bamboo PRIVATE -static-libgcc -static-libstdc++)
set_target_properties(bamboo PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(bamboo PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
