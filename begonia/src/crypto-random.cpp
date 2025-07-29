#include "palm/crypto.hpp"

#include <algorithm>
#include <functional>
#include <iomanip>
#include <iterator>
#include <sstream>
#include <string>

#include <boost/lexical_cast.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <cppcodec/base32_crockford.hpp>
#include <cppcodec/base64_rfc4648.hpp>
#include <cppcodec/base64_url_unpadded.hpp>
#include <msgpack.hpp>

std::string palm::base64::to_string(const std::vector<uint8_t> buf) {
  return cppcodec::base64_rfc4648::encode(buf);
}
std::vector<uint8_t> palm::base64::from_string(const std::string& str) {
  return cppcodec::base64_rfc4648::decode(str);
}

std::vector<uint8_t> palm::random::bytes(size_t len) {
  static std::mt19937 rng(std::time(nullptr));
  std::uniform_int_distribution<uint8_t> dist(
      std::numeric_limits<uint8_t>::min(), std::numeric_limits<uint8_t>::max());

  std::vector<uint8_t> buf;
  for (auto i = 0; i < len; i++) {
    buf.push_back(dist(rng));
  }
  return buf;
}

std::string palm::random::alphanumeric(size_t len) {
  static std::mt19937 rng(std::time(nullptr));
  // static const char CHARSET[] =
  //     "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  static const std::string CHARSET =
      "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

  std::string it;
  it.resize(len);
  // std::sample(std::cbegin(CHARSET), std::cend(CHARSET), std::begin(it),
  //             std::intptr_t(len), std::forward<std::mt19937>(rng));
  std::sample(std::begin(CHARSET), std::end(CHARSET), std::begin(it),
              std::intptr_t(len), std::forward<std::mt19937>(rng));
  return it;
}

std::string palm::uuid() {
  static boost::uuids::random_generator gen;
  boost::uuids::uuid it = gen();
  return boost::lexical_cast<std::string>(it);
}

// https://cplusplus.com/reference/iomanip/put_time/
std::string palm::timestamp(std::time_t it) {
  std::stringstream ss;
  struct std::tm* tm = std::localtime(&it);
  ss << std::put_time(tm, "%Y%m%d%H%M%S");
  return ss.str();
}

std::pair<std::string, double> palm::salted_password::verify(
    const std::string& code) {
  const auto buf = cppcodec::base64_url_unpadded::decode(code);

  const std::string str(buf.begin(), buf.end());
  msgpack::object_handle oh = msgpack::unpack(str.data(), str.size());
  msgpack::object obj = oh.get();
  msgpack::type::tuple<std::string, double> it;
  obj.convert(it);

  std::string plain;
  double salt;
  std::tie(plain, salt) = it;
  std::pair<std::string, double> ret(plain, salt);
  return ret;
}

std::string palm::salted_password::sign(const std::string& plain) {
  const auto salt = palm::random::double_();
  msgpack::type::tuple<std::string, double> src(plain, salt);
  std::stringstream buf;
  msgpack::pack(buf, src);
  buf.seekg(0);
  return cppcodec::base64_url_unpadded::encode(buf.str());
}
