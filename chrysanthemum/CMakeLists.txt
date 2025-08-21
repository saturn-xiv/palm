file(GLOB CHRYSANTHEMUM_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(chrysanthemum ${CHRYSANTHEMUM_SOURCES})
target_link_libraries(chrysanthemum PRIVATE begonia)
target_link_options(chrysanthemum PRIVATE -static-libgcc -static-libstdc++)
set_target_properties(chrysanthemum PROPERTIES LINK_FLAGS_RELEASE -s)    

target_include_directories(chrysanthemum PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
)
