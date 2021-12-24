#include "palm/crypto.hpp"

#include <algorithm>
#include <climits>
#include <functional>
#include <iomanip>
#include <random>
#include <sstream>

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

std::vector<uint8_t> palm::Hmac::sum(const EVP_MD* engine,
                                     const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt,
                                     const size_t len) {
  unsigned char it[len];

  // std::vector<uint8_t> buf;
  // {
  //   buf.insert(buf.end(), plain.begin(), plain.end());
  //   buf.insert(buf.end(), salt.begin(), salt.end());
  // }
  HMAC_CTX* hmac = HMAC_CTX_new();
  HMAC_Init_ex(hmac, &*this->key.begin(), this->key.size(), engine, NULL);
  HMAC_Update(hmac, &*plain.begin(), plain.size());
  HMAC_Update(hmac, &*salt.begin(), salt.size());

  unsigned int l = len;
  HMAC_Final(hmac, it, &l);
  HMAC_CTX_free(hmac);

  std::vector<uint8_t> tmp(it, it + len);
  return tmp;
}

palm::Hmac::Hmac(const std::string& key) {
  this->key = palm::base64::from(key);
}
bool palm::ssha512::verify(const std::string& secret,
                           const std::string& plain) {
  if (secret.size() <= HEADER.size()) {
    return false;
  }

  const auto buf = palm::base64::from(secret.substr(HEADER.size()));
  if (buf.size() <= SHA512_DIGEST_LENGTH) {
    return false;
  }
  const std::vector<uint8_t> salt = {buf.begin() + SHA512_DIGEST_LENGTH,
                                     buf.end()};
  return palm::ssha512::sum(plain, salt) == secret;
}

std::string palm::ssha512::sum(const std::string& plain,
                               const std::vector<uint8_t>& salt) {
  uint8_t hash[SHA512_DIGEST_LENGTH];
  SHA512_CTX ctx;
  SHA512_Init(&ctx);
  SHA512_Update(&ctx, plain.c_str(), plain.size());
  SHA512_Update(&ctx, &*salt.begin(), salt.size());
  SHA512_Final(hash, &ctx);

  std::vector<uint8_t> buf;
  {
    std::copy(hash, hash + SHA512_DIGEST_LENGTH, std::back_inserter(buf));
    buf.insert(buf.end(), salt.begin(), salt.end());
  }
  std::stringstream ss;
  ss << HEADER << palm::base64::to(buf);

  return ss.str();
}
std::string palm::ssha512::sum(const std::string& plain,
                               const size_t salt_len) {
  std::vector<uint8_t> salt = palm::random::bytes(salt_len);
  return palm::ssha512::sum(plain, salt);
}
