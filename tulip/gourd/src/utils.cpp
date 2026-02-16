#include "palm/utils.hpp"

#include <climits>
#include <filesystem>
#include <iomanip>
#include <random>

#include <boost/uuid/random_generator.hpp>
#include <boost/uuid/string_generator.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <cppcodec/base64_url_unpadded.hpp>

std::vector<uint8_t> palm::base64::from(const std::string& str) {
  return cppcodec::base64_url_unpadded::decode(str);
}

std::string palm::base64::to(const std::vector<uint8_t>& buf) {
  return cppcodec::base64_url_unpadded::encode(buf);
}

std::string palm::uuid() {
  boost::uuids::uuid it = boost::uuids::random_generator()();
  return boost::uuids::to_string(it);
}

std::vector<uint8_t> palm::random::bytes(size_t len) {
  thread_local static std::mt19937 rg{std::random_device{}()};
  thread_local static std::independent_bits_engine<std::mt19937, CHAR_BIT,
                                                   uint8_t>
      pick(rg);
  std::vector<uint8_t> buf(len);
  std::generate(buf.begin(), buf.end(), std::ref(pick));
  return buf;
}

std::string palm::random::alphanumeric(size_t len) {
  static const std::string CHARSETS =
      R"(0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ)";

  thread_local static std::mt19937 rg{std::random_device{}()};
  thread_local static std::uniform_int_distribution<std::string::size_type>
      pick(0, CHARSETS.length() - 1);

  std::string it;
  it.reserve(len);
  for (size_t i = 0; i < len; ++i) {
    it += CHARSETS[pick(rg)];
  }

  return it;
}

bool palm::is_stopped() {
  static const std::string file = ".stop";
  const auto ok = std::filesystem::exists(file);
  if (ok) {
    spdlog::warn("file {} exists, will be exited...", file);
  }
  return ok;
}
