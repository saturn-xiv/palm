#include "palm/crypto.hpp"

#include <algorithm>
#include <climits>
#include <functional>
#include <random>

#include <boost/lexical_cast.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <cppcodec/base64_rfc4648.hpp>

std::string palm::uuid() {
  boost::uuids::uuid it = boost::uuids::random_generator()();
  return boost::lexical_cast<std::string>(it);
}

std::vector<uint8_t> palm::random::bytes(const size_t len) {
  std::random_device rd;
  std::mt19937 mt(rd());
  std::uniform_int_distribution<uint8_t> dist(0, 255);

  std::vector<unsigned char> buf;
  auto gen = std::bind(dist, mt);
  buf.resize(len);
  std::generate(buf.begin(), buf.end(), gen);
  return std::move(buf);
}

std::string palm::random::string(const size_t len) {
  const std::string chars = "0123456789abcdefghijklmnopqrstuvwxyz";
  std::random_device rd;
  std::mt19937 mt(rd());
  std::uniform_int_distribution<uint8_t> dist(0, chars.size() - 1);
  std::stringstream ss;
  for (auto i = 0; i < len; i++) {
    ss << chars[dist(mt)];
  }
  return ss.str();
}

std::string palm::base64::to(const std::vector<uint8_t>& buf) {
  return cppcodec::base64_rfc4648::encode(buf);
}
std::vector<uint8_t> palm::base64::from(const std::string& str) {
  return cppcodec::base64_rfc4648::decode(str);
}
