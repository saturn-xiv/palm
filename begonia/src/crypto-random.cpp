#include "basil/crypto.hpp"

#include <algorithm>
#include <climits>
#include <functional>
#include <iomanip>
#include <iterator>
#include <random>
#include <sstream>
#include <string>

#include <boost/lexical_cast.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <cppcodec/base32_crockford.hpp>
#include <cppcodec/base64_rfc4648.hpp>

std::string basil::base64::to_string(const std::vector<uint8_t> buf) {
  return cppcodec::base64_rfc4648::encode(buf);
}
std::vector<uint8_t> basil::base64::from_string(const std::string& str) {
  return cppcodec::base64_rfc4648::decode(str);
}

std::vector<uint8_t> basil::random::bytes(size_t len) {
  static std::mt19937 rng(std::time(nullptr));
  std::uniform_int_distribution<uint8_t> dist(
      std::numeric_limits<uint8_t>::min(), std::numeric_limits<uint8_t>::max());

  std::vector<uint8_t> buf;
  for (auto i = 0; i < len; i++) {
    buf.push_back(dist(rng));
  }
  return buf;
}
std::string basil::random::alphanumeric(size_t len) {
  static std::mt19937 rng(std::time(nullptr));
  static const char CHARSET[] =
      "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

  std::string it;
  it.resize(len);
  std::sample(std::cbegin(CHARSET), std::cend(CHARSET), std::begin(it),
              std::intptr_t(len), std::forward<std::mt19937>(rng));
  return it;
}

std::string basil::uuid() {
  static boost::uuids::random_generator gen;
  boost::uuids::uuid it = gen();
  return boost::lexical_cast<std::string>(it);
}

// https://cplusplus.com/reference/iomanip/put_time/
std::string basil::timestamp(std::time_t it) {
  std::stringstream ss;
  struct std::tm* tm = std::localtime(&it);
  ss << std::put_time(tm, "%Y%m%d%H%M%S");
  return ss.str();
}
