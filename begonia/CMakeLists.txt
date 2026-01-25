file(GLOB BEGONIA_SOURCE_LIST CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")
file(GLOB GOURD_SOURCE_LIST CONFIGURE_DEPENDS "${PROJECT_SOURCE_DIR}/gourd/src/*")

# -----------------------------------------------------------------------------
find_package(OpenSSL REQUIRED)
find_package(SQLite3 REQUIRED)
find_package(PostgreSQL REQUIRED)
find_package(CURL REQUIRED)
find_package(rabbitmq-c REQUIRED)
find_package(Boost REQUIRED COMPONENTS date_time)

find_package(PkgConfig REQUIRED)
pkg_check_modules(MYSQLCLIENT REQUIRED IMPORTED_TARGET "mysqlclient")
# -----------------------------------------------------------------------------
FetchContent_Declare(argparse
  GIT_REPOSITORY "https://github.com/p-ranav/argparse.git"
  GIT_TAG        "v3.2"
)

FetchContent_Declare(spdlog
  GIT_REPOSITORY "https://github.com/gabime/spdlog.git"
  GIT_TAG        "v1.16.0"
)

FetchContent_Declare(tomlplusplus
  GIT_REPOSITORY "https://github.com/marzer/tomlplusplus.git"
  GIT_TAG        "v3.4.0"
)

FetchContent_Declare(cppcodec
  GIT_REPOSITORY "https://github.com/tplgy/cppcodec.git"
  GIT_TAG        "8019b8b"
)

FetchContent_Declare(inja
  GIT_REPOSITORY "https://github.com/pantor/inja.git"
  GIT_TAG        "v3.5.0"
)

FetchContent_Declare(cpphttplib
  GIT_REPOSITORY "https://github.com/yhirose/cpp-httplib.git"
  GIT_TAG        "v0.28.0"
)

FetchContent_Declare(redisplusplus
  GIT_REPOSITORY "https://github.com/sewenew/redis-plus-plus.git"
  GIT_TAG        "1.3.15"
)

set(WITH_POSTGRESQL ON)
set(WITH_MYSQL ON)
set(WITH_SQLITE3 ON)
set(WITH_BOOST ON)
FetchContent_Declare(soci
  GIT_REPOSITORY "https://github.com/SOCI/soci.git"
  GIT_TAG        "v4.1.2"
)

set(CPR_USE_SYSTEM_CURL ON)
FetchContent_Declare(cpr
  GIT_REPOSITORY "https://github.com/libcpr/cpr.git"
  GIT_TAG        "1.14.1"
)

set(ABSL_ENABLE_INSTALL ON)
FetchContent_Declare(absl
  GIT_REPOSITORY "https://github.com/abseil/abseil-cpp.git"
  GIT_TAG        "20250814.1"
)

set(gRPC_SSL_PROVIDER package)
FetchContent_Declare(gRPC
  GIT_REPOSITORY "https://github.com/grpc/grpc.git"
  GIT_TAG        "v1.76.0"
)

FetchContent_MakeAvailable(argparse spdlog tomlplusplus cppcodec inja cpphttplib soci redisplusplus cpr absl gRPC)
# -----------------------------------------------------------------------------

add_library(begonia STATIC ${BEGONIA_SOURCE_LIST} ${GOURD_SOURCE_LIST}) 

target_link_libraries(begonia PUBLIC m dl anl pthread
  ${OPENSSL_LIBRARIES} ${Boost_LIBRARIES}
  PostgreSQL::PostgreSQL PkgConfig::MYSQLCLIENT SQLite::SQLite3 
  CURL::libcurl rabbitmq::rabbitmq

  argparse cpr::cpr tomlplusplus::tomlplusplus
  grpc++  
  redis++  
  soci_core soci_postgresql soci_mysql soci_sqlite3
)

target_include_directories(begonia PUBLIC
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${PROJECT_SOURCE_DIR}/gourd/include

  ${cpphttplib_SOURCE_DIR}
  ${cppcodec_SOURCE_DIR}
  ${spdlog_SOURCE_DIR}/include
  ${cpr_SOURCE_DIR}/include
  ${soci_SOURCE_DIR}/include
  ${tomlplusplus_SOURCE_DIR}/include
  ${inja_SOURCE_DIR}/single_include
  ${inja_SOURCE_DIR}/third_party/include  
)
