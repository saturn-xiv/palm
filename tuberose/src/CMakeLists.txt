file(GLOB HEADER_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/include/palm/*.hpp")
file(GLOB SOURCE_LIST CONFIGURE_DEPENDS "${palm_SOURCE_DIR}/src/*.cpp")

add_library(palm STATIC ${SOURCE_LIST})

find_package(Boost REQUIRED 
  chrono date_time timer log log_setup
  filesystem system thread program_options
  random regex exception json
)
find_package(cppzmq CONFIG REQUIRED)
find_package(PahoMqttCpp CONFIG REQUIRED)
find_package(rabbitmq-c CONFIG REQUIRED)
find_package(cpr CONFIG REQUIRED)
find_package(aws-cpp-sdk-s3 CONFIG REQUIRED)
find_package(aws-cpp-sdk-sqs CONFIG REQUIRED)
find_package(Libssh2 CONFIG REQUIRED)
find_package(LibLZMA REQUIRED)
find_package(SailC++ CONFIG REQUIRED)
find_package(msgpack-c CONFIG REQUIRED)
find_package(pugixml CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(yaml-cpp CONFIG REQUIRED)
find_package(nlohmann_json CONFIG REQUIRED)
find_package(mstch CONFIG REQUIRED)
find_package(inja CONFIG REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(libpqxx CONFIG REQUIRED)
find_package(unofficial-sodium CONFIG REQUIRED)
find_package(unofficial-libmariadb CONFIG REQUIRED)
find_package(unofficial-sqlite3 CONFIG REQUIRED)
find_package(SQLiteCpp CONFIG REQUIRED)
find_package(redis++ CONFIG REQUIRED)
find_package(SOCI CONFIG REQUIRED)
find_package(Flatbuffers CONFIG REQUIRED)
find_package(fmt CONFIG REQUIRED)
find_package(miniocpp CONFIG REQUIRED)

pkg_check_modules(tomlplusplus REQUIRED IMPORTED_TARGET tomlplusplus)

find_path(CPPCODEC_INCLUDE_DIRS "cppcodec/base32_crockford.hpp")
find_path(CPP_HTTPLIB_INCLUDE_DIRS "httplib.h")

find_library(CURLPP_LIBRARY curlpp)
find_library(INIH_LIBRARY inih)

FetchContent_Declare(casbin
  GIT_REPOSITORY    "https://github.com/casbin/casbin-cpp.git"
  GIT_TAG           "v1.59.0"
  PATCH_COMMAND     git apply "${palm_SOURCE_DIR}/casbin.patch"
)
FetchContent_MakeAvailable(casbin)

target_link_libraries(palm PUBLIC 
  lemon gourd casbin

  Boost::chrono Boost::date_time Boost::timer Boost::log Boost::log_setup
  Boost::filesystem Boost::system Boost::thread Boost::program_options
  Boost::random Boost::regex Boost::exception Boost::json

  unofficial-sodium::sodium Libssh2::libssh2_static LibLZMA::LibLZMA

  cppzmq-static PahoMqttCpp::paho-mqttpp3-static rabbitmq::rabbitmq-static
    
  aws-cpp-sdk-s3 aws-cpp-sdk-sqs 
  ${MINIO_LIBRARY} CURL::libcurl ${CURLPP_LIBRARY} ${INIH_LIBRARY}
  cpr::cpr miniocpp::miniocpp SAIL::sail-c++
  
  nlohmann_json::nlohmann_json PkgConfig::tomlplusplus pugixml::static yaml-cpp msgpack-c
  mstch::mstch pantor::inja
  PostgreSQL::PostgreSQL libpqxx::pqxx 
  unofficial::libmariadb # mariadbcpp
  unofficial::sqlite3::sqlite3 SQLiteCpp
  SOCI::soci_core_static SOCI::soci_empty_static SOCI::soci_postgresql_static
  redis++::redis++_static 
    
  flatbuffers::flatbuffers

  fmt::fmt-header-only spdlog::spdlog_header_only
)

target_include_directories(palm PUBLIC
  ${palm_SOURCE_DIR}/include

  ${CPPCODEC_INCLUDE_DIRS}
  ${CPP_HTTPLIB_INCLUDE_DIRS}
  ${casbin_SOURCE_DIR}/include
)

source_group(
  TREE "${PROJECT_SOURCE_DIR}/include"
  PREFIX "Header Files"
  FILES ${HEADER_LIST}
)

