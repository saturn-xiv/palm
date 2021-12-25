#pragma once

#include <chrono>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#include <openssl/hmac.h>
#include <openssl/md5.h>
#include <openssl/sha.h>
#include <nlohmann/json.hpp>

// openssl rand -base64 32
namespace palm {
std::string uuid();

namespace random {
std::vector<uint8_t> bytes(const size_t len);
std::string string(const size_t len);
}  // namespace random
namespace base64 {
std::string to(const std::vector<uint8_t>& buf);
std::vector<uint8_t> from(const std::string& str);
}  // namespace base64

class Hmac {
 public:
  Hmac(const std::string& key);
  inline std::vector<uint8_t> sha512(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) const {
    return this->sum(EVP_sha512(), plain, salt, SHA512_DIGEST_LENGTH);
  }
  inline std::vector<uint8_t> sha384(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) const {
    return this->sum(EVP_sha384(), plain, salt, SHA384_DIGEST_LENGTH);
  }
  inline std::vector<uint8_t> sha256(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) const {
    return this->sum(EVP_sha256(), plain, salt, SHA256_DIGEST_LENGTH);
  }
  inline std::vector<uint8_t> sha224(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) const {
    return this->sum(EVP_sha224(), plain, salt, SHA224_DIGEST_LENGTH);
  }

  inline std::vector<uint8_t> md5(const std::vector<uint8_t>& plain,
                                  const std::vector<uint8_t>& salt) const {
    return this->sum(EVP_md5(), plain, salt, MD5_DIGEST_LENGTH);
  }

 private:
  std::vector<uint8_t> sum(const EVP_MD* engine,
                           const std::vector<uint8_t>& plain,
                           const std::vector<uint8_t>& salt,
                           const size_t len) const;

  std::vector<uint8_t> key;
};
namespace ssha512 {

// https://wiki.dovecot.org/HowTo/ConvertPasswordSchemes
// https://mad9scientist.com/dovecot-password-creation-php/
std::string sum(const std::string& plain, const size_t salt_len);
std::string sum(const std::string& plain, const std::vector<uint8_t>& salt);
bool verify(const std::string& secret, const std::string& plain);
const static std::string HEADER = "{SSHA512}";
}  // namespace ssha512

class Jwt {
 public:
  Jwt(const std::string& key) : key(palm::base64::from(key)) {}
  std::string encode(const std::string& audience, const std::string& subject,
                     const nlohmann::json& payload,
                     const std::chrono::seconds& ttl) const;
  std::tuple<std::string, std::string, nlohmann::json> decode(
      const std::string& token) const;

 private:
  std::string signature(const std::string& header,
                        const std::string& payload) const;

  inline static const std::string AUTHORIZATION = "Authorization";
  inline static const std::string BEARER = "Bearer";
  inline static const std::string POT = ".";
  inline static const std::string AUDIENCE = "aud";
  inline static const std::string SUBJECT = "sub";
  inline static const std::string EXPIRE = "exp";
  inline static const std::string NOT_BEFORE = "nbf";
  std::vector<uint8_t> key;
};
// https://wiki.openssl.org/index.php/EVP_Symmetric_Encryption_and_Decryption
class Aes {
 public:
  Aes(const std::string& key);

  std::pair<std::vector<uint8_t>, std::vector<uint8_t>> encrypt(
      const std::vector<uint8_t>& plain) const;
  std::vector<uint8_t> decrypt(const std::vector<uint8_t>& code,
                               const std::vector<uint8_t>& iv) const;

 private:
  std::vector<uint8_t> key;
  static const size_t KEY_SIZE = 256 / 8;
  static const size_t BLOCK_SIZE = 128 / 8;
};
}  // namespace palm
