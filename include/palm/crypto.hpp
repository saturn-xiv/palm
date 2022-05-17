#pragma once

#include <chrono>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include <boost/property_tree/ptree.hpp>

#include <Poco/Crypto/DigestEngine.h>
#include <openssl/hmac.h>
#include <openssl/md5.h>
#include <openssl/sha.h>

namespace palm {

std::string uuid();

namespace random {
std::string alphanumeric(const size_t len);
std::vector<uint8_t> bytes(const size_t len);
}  // namespace random

namespace base64 {
std::string encode(const std::vector<uint8_t>& buf);
std::vector<uint8_t> decode(const std::string& str);
}  // namespace base64

class Jwt {
 public:
  Jwt(const boost::property_tree::ptree& config);
  Jwt(const std::string& key) : key(key) {}
  std::string sign(
      const std::string& subject,
      const std::unordered_map<std::string, std::string>& payload,
      const std::chrono::seconds& ttl = std::chrono::days(1)) const;
  std::pair<std::string, std::unordered_map<std::string, std::string>> verify(
      const std::string& token) const;

 private:
  std::string key;
};

class Aes {
 public:
  Aes(const std::string& key) : key(key) {}
  std::pair<std::string, std::string> encrypt(const std::string& plain,
                                              const size_t salt_len = 32) const;
  std::string decrypt(const std::string& code, const std::string& salt) const;

 private:
  std::string key;
  inline const static std::string CIPHER_KEY_NAME = "aes-256-cbc";
};

class HMac {
 public:
  HMac(const std::string& key) : key(key) {}
  std::vector<uint8_t> sign(const std::vector<uint8_t>& plain,
                            const size_t salt_len = 16) const;
  bool verify(const std::vector<uint8_t>& code,
              const std::vector<uint8_t>& plain) const;

 private:
  std::vector<uint8_t> sign(const std::vector<uint8_t>& plain,
                            const std::vector<uint8_t>& salt) const;
  std::string key;
};

class SHA512Engine : public Poco::Crypto::DigestEngine {
 public:
  enum { BLOCK_SIZE = 64, DIGEST_SIZE = SHA512_DIGEST_LENGTH };

  SHA512Engine() : Poco::Crypto::DigestEngine("SHA512") {}
};

namespace ssha512 {
// https://wiki.dovecot.org/HowTo/ConvertPasswordSchemes
// https://mad9scientist.com/dovecot-password-creation-php/

const static std::string HEADER = "{SSHA512}";

std::string sign(const std::string& plain, const size_t salt_len);

bool verify(const std::string& code, const std::string& plain);

std::string sign(const std::string& plain, const std::string& salt);

}  // namespace ssha512

}  // namespace palm
