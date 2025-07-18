#include "palm/utils.hpp"
#include "palm/cache.hpp"
#include "palm/crypto.hpp"
#include "palm/orm.hpp"
#include "palm/queue.hpp"
#include "palm/s3.hpp"
#include "palm/version.hpp"

#include <cstdlib>
#include <fstream>

#include <boost/algorithm/hex.hpp>
#include <boost/algorithm/string.hpp>
#include <boost/algorithm/string/join.hpp>

#include <curl/curl.h>
#include <google/protobuf/stubs/common.h>
#include <grpcpp/grpcpp.h>
#include <openssl/opensslv.h>
#include <sodium.h>
#include <thrift/version.h>

// #if BOOST_ARCH_X86_64
// #include <mysql/mariadb_version.h>
// #endif

void palm::init(bool debug) {
  spdlog::set_level(debug ? spdlog::level::debug : spdlog::level::info);

  spdlog::debug("run on debug mode({})", palm::GIT_VERSION);
  spdlog::debug("{}", OPENSSL_VERSION_TEXT);
  {
    const auto v = PQlibVersion();
    spdlog::debug("PostgreSQL v{}.{}.{}", v / (100 * 100), (v / 100) % 100,
                  v % (100 * 100));
  }
  // #if BOOST_ARCH_X86_64
  //   spdlog::debug("MySQL v{}", MARIADB_CLIENT_VERSION_STR);
  // #endif
  spdlog::debug("Sqlite v{}", SQLITE_VERSION);
  spdlog::debug("{}", curl_version());
  spdlog::debug("rabbitmq-c v{}", AMQ_VERSION_STRING);
  spdlog::debug("hiredis v{}.{}.{}", HIREDIS_MAJOR, HIREDIS_MINOR,
                HIREDIS_PATCH);
  spdlog::debug("miniocpp v{}", MINIO_CPP_VERSION);
  spdlog::debug("protobuf v{}", google::protobuf::internal::VersionString(
                                    GOOGLE_PROTOBUF_VERSION));
  spdlog::debug("gRpc v{}", grpc::Version());
  spdlog::debug("thrift v{}", THRIFT_VERSION);
  {
    if (sodium_init() < 0) {
      spdlog::error("the sodium library couldn't be initialized");
      std::exit(EXIT_FAILURE);
    }
    spdlog::debug("sodium v{}", SODIUM_VERSION_STRING);
  }
}

// https://docs.gravatar.com/api/avatars/hash/
std::string palm::gravatar::hash(const std::string& email) {
  std::string e = boost::trim_copy(email);
  boost::algorithm::to_lower(e);
  auto d = palm::sha256::sign(e);

  std::string h;
  boost::algorithm::hex(d.begin(), d.end(), std::back_inserter(h));
  boost::algorithm::to_lower(h);
  return h;
}

void palm::load(const std::filesystem::path& f, std::string& s) {
  std::ifstream file;
  file.exceptions(std::ifstream::failbit | std::ifstream::badbit);
  file.open(f, std::ios_base::binary);
  std::size_t size = static_cast<std::size_t>(std::filesystem::file_size(f));
  s.resize(size, '\0');
  file.read(&s[0], size);
}
void palm::load(const std::filesystem::path& f, std::vector<uint8_t> b) {
  std::ifstream file(f, std::ios::binary);
  b.reserve(static_cast<std::size_t>(std::filesystem::file_size(f)));
  std::copy(std::istream_iterator<uint8_t>(file),
            std::istream_iterator<uint8_t>(), std::back_inserter(b));
}
