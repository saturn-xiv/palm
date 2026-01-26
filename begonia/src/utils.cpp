#include "palm/utils.hpp"

#include <gnu/libc-version.h>

#include <amqp.h>
#include <curl/curl.h>
#include <google/protobuf/stubs/common.h>
#include <grpcpp/grpcpp.h>
#include <hiredis/hiredis.h>
#include <httplib.h>
#include <libpq-fe.h>
#include <mysql.h>
#include <openssl/ssl.h>
#include <soci/version.h>
#include <sqlite3.h>
#include <boost/version.hpp>

void palm::init(bool debug) {
  spdlog::set_level(debug ? spdlog::level::debug : spdlog::level::info);

  // https://gcc.gnu.org/onlinedocs/cpp/Predefined-Macros.html
  {
    spdlog::debug("Compiled on {} at {}", __DATE__, __TIME__);
    spdlog::debug("c++: {}", __cplusplus);
#ifdef __GNUC__
    // spdlog::debug("GCC: v{}.{}.{}", __GNUC__, __GNUC_MINOR__,
    //               __GNUC_PATCHLEVEL__);
    spdlog::debug("GCC: v{}", __VERSION__);
#endif
#ifdef __clang__
    spdlog::debug("Clang: v{}.{}.{}", __clang_major__, __clang_minor__,
                  __clang_patchlevel__);
#endif
  }

  spdlog::debug("Glibc: v{}", gnu_get_libc_version());
  spdlog::debug("Boost: v{}.{}.{}", BOOST_VERSION / 100000,
                BOOST_VERSION / 100 % 1000, BOOST_VERSION % 100);
  {
    SSL_library_init();
    spdlog::debug("{}", OpenSSL_version(SSLEAY_VERSION));

    // spdlog::debug("OpenSSL: v{}", OPENSSL_FULL_VERSION_STR);
  }
  {
    // const auto ver = curl_version_info(CURLVERSION_NOW);
    // if (ver) {
    //   spdlog::debug("libcurl: v{}", ver->version);
    // }
    spdlog::debug("{}", curl_version());

    // spdlog::debug("libcurl: v{}", LIBCURL_VERSION);
  }
  {
    const auto ver = PQlibVersion();
    spdlog::debug("PostgreSql: v{}.{}.{}", ver / 10000, (ver % 10000) / 100,
                  ver % 100);
  }
  spdlog::debug("MySQL: v{}", mysql_get_client_info());
  spdlog::debug("Sqlite3: v{}", sqlite3_libversion());
  spdlog::debug("SOCI: v{}.{}.{}", SOCI_VERSION / 100000,
                SOCI_VERSION / 100 % 1000, SOCI_VERSION % 100);
  spdlog::debug("Redis: v{}.{}.{}", HIREDIS_MAJOR, HIREDIS_MINOR,
                HIREDIS_PATCH);
  spdlog::debug("RabbitMQ: v{}", amqp_version());
  spdlog::debug("cpp-httplib: v{}", CPPHTTPLIB_VERSION);
  spdlog::debug(
      "Protocol Buffers: v{}",
      google::protobuf::internal::VersionString(GOOGLE_PROTOBUF_VERSION));
  spdlog::debug("gRPC: v{}", grpc::Version());
}
