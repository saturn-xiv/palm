file(GLOB TEST_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/*.cpp)

find_package(Catch2 CONFIG REQUIRED)

foreach(sfn ${TEST_SOURCES})
    file(RELATIVE_PATH itn ${CMAKE_CURRENT_SOURCE_DIR} ${sfn})
    string(REPLACE ".cpp" "-test" it ${itn})
    add_executable(${it} ${itn})

    target_link_libraries(${it} PRIVATE palm Catch2::Catch2WithMain)
    set_target_properties(${it} PROPERTIES LINK_FLAGS_RELEASE -s)
endforeach(sfn ${TEST_SOURCES})

