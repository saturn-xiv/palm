#include "palm/crypto.hpp"

#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/hmac.h>
#include <openssl/opensslv.h>
#include <openssl/pem.h>
#include <openssl/sha.h>
#include <cppcodec/base64_url.hpp>

// https://en.cppreference.com/w/cpp/io/manip/put_time
std::string palm::timestamp() {
  auto now = std::chrono::system_clock::now();
  auto itt = std::chrono::system_clock::to_time_t(now);
  std::ostringstream ss;
  ss << std::put_time(gmtime(&itt), "%Y%m%d%H%M%S");
  return ss.str();
}

std::string palm::uuid::v4() {
  // #include <Poco/UUID.h>
  // #include <Poco/UUIDGenerator.h>
  // thread_local static Poco::UUIDGenerator& gen =
  //     Poco::UUIDGenerator::defaultGenerator();
  // Poco::UUID it = gen.create();
  // return it.toString();

  std::ifstream it("/proc/sys/kernel/random/uuid");
  return std::string((std::istreambuf_iterator<char>(it)),
                     std::istreambuf_iterator<char>());
}

std::string palm::random::string(const std::string::size_type len) {
  std::stringstream ss;
  std::uniform_int_distribution<int> ud(32, 126);
  std::random_device rd;
  for (auto i = 0; i < len; i++) {
    ss << (char)ud(rd);
  }
  return ss.str();
}

std::vector<uint8_t> palm::random::bytes(const size_t len) {
  std::random_device rd;
  static std::independent_bits_engine<std::default_random_engine, CHAR_BIT,
                                      uint8_t>
      rbe(rd());
  std::vector<uint8_t> buf(len);
  std::generate(begin(buf), end(buf), std::ref(rbe));
  return buf;
}

std::string palm::hex::to(const std::vector<uint8_t>& buffer) {
  std::stringstream ss;
  ss << std::hex << std::setfill('0');
  for (const auto it : buffer) {
    ss << std::setw(2) << static_cast<unsigned>(it);
  }
  return ss.str();
}
std::vector<uint8_t> palm::hex::from(const std::string& buffer) {
  std::vector<uint8_t> items;
  for (auto i = 0; i < buffer.length(); i += 2) {
    const auto it = buffer.substr(i, 2);
    items.push_back(static_cast<uint8_t>(strtol(it.c_str(), NULL, 16)));
  }
  return items;
}
std::string palm::base64::to(const std::vector<uint8_t>& buffer) {
  return cppcodec::base64_url::encode(buffer);
}
std::vector<uint8_t> palm::base64::from(const std::string& buffer) {
  return cppcodec::base64_url::decode(buffer);
}

palm::Session::Session(grpc::ServerContext* ctx) {
  this->peer = ctx->peer();

  const auto mt = ctx->client_metadata();
  {
    const auto it = mt.find(AUTHORIZAATION);
    if (it != mt.end()) {
      if (it->second.starts_with(BEARER)) {
        const auto auth = it->second.substr(BEARER.length());
        this->token = std::string(auth.data());
      }
    }
  }
}
