#pragma once

#include <memory>
#include <string>
#include <vector>

#include <openssl/hmac.h>
#include <openssl/md5.h>
#include <openssl/sha.h>

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
                                     const std::vector<uint8_t>& salt) {
    return this->sum(EVP_sha512(), plain, salt, SHA512_DIGEST_LENGTH);
  }
  inline std::vector<uint8_t> sha384(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) {
    return this->sum(EVP_sha384(), plain, salt, SHA384_DIGEST_LENGTH);
  }
  inline std::vector<uint8_t> sha256(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) {
    return this->sum(EVP_sha256(), plain, salt, SHA256_DIGEST_LENGTH);
  }
  inline std::vector<uint8_t> sha224(const std::vector<uint8_t>& plain,
                                     const std::vector<uint8_t>& salt) {
    return this->sum(EVP_sha224(), plain, salt, SHA224_DIGEST_LENGTH);
  }

  inline std::vector<uint8_t> md5(const std::vector<uint8_t>& plain,
                                  const std::vector<uint8_t>& salt) {
    return this->sum(EVP_md5(), plain, salt, MD5_DIGEST_LENGTH);
  }

 private:
  std::vector<uint8_t> sum(const EVP_MD* engine,
                           const std::vector<uint8_t>& plain,
                           const std::vector<uint8_t>& salt, const size_t len);

  std::vector<uint8_t> key;
};
namespace ssha512 {

std::vector<uint8_t> sum(const std::vector<uint8_t>& plain,
                         const size_t salt_len);
}  // namespace ssha512
// https://wiki.dovecot.org/HowTo/ConvertPasswordSchemes
// https://mad9scientist.com/dovecot-password-creation-php/
std::string ssha(const EVP_MD* engine, const std::string& plain, const size_t);
class Jwt {};
class Aes {};
}  // namespace palm
