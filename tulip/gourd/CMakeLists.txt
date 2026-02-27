file(GLOB PROTOCOLS_SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cc)
file(GLOB GOURD_SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
# -----------------------------------------------------------------------------
execute_process(COMMAND git describe --tags --always --dirty
    OUTPUT_VARIABLE GIT_REV
    ERROR_QUIET
)
string(STRIP "${GIT_REV}" GIT_REV)
string(TIMESTAMP BUILD_TIME UTC)
configure_file(include/palm/version.h.in ${CMAKE_CURRENT_SOURCE_DIR}/include/palm/version.hpp)
# -----------------------------------------------------------------------------
find_package(boost_algorithm CONFIG REQUIRED)
find_package(boost_uuid CONFIG REQUIRED)
find_package(boost_date_time CONFIG REQUIRED)
find_package(boost_filesystem CONFIG REQUIRED)
find_package(boost_url CONFIG REQUIRED)
find_package(boost_beast CONFIG REQUIRED)
find_package(boost_typeof CONFIG REQUIRED)
find_package(boost_exception CONFIG REQUIRED)

find_package(argparse CONFIG REQUIRED)
find_package(httplib CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(redis++ CONFIG REQUIRED)
find_package(SOCI CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(tomlplusplus CONFIG REQUIRED)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")
# -----------------------------------------------------------------------------
add_library(gourd STATIC ${PROTOCOLS_SOURCES} ${GOURD_SOURCE_LIST}) 

target_link_libraries(gourd PUBLIC  m dl anl pthread
  Boost::algorithm Boost::uuid Boost::date_time Boost::filesystem Boost::url Boost::beast Boost::typeof Boost::exception
  argparse::argparse spdlog::spdlog_header_only cpr::cpr
  rabbitmq::rabbitmq-static redis++::redis++_static
  SOCI::soci_core_static SOCI::soci_postgresql_static
  nlohmann_json::nlohmann_json pantor::inja httplib::httplib tomlplusplus::tomlplusplus
  gRPC::grpc++
)

target_include_directories(gourd PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include  
)
