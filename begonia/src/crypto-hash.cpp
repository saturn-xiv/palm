#include "palm/crypto.hpp"

#include <boost/algorithm/hex.hpp>

#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/md5.h>
#include <openssl/sha.h>
#include <spdlog/spdlog.h>
#include <cppcodec/base64_rfc4648.hpp>

palm::HMac::HMac(const std::string &key)
    : _key(cppcodec::base64_rfc4648::decode(key)) {}
std::vector<uint8_t> palm::HMac::sign(const std::vector<uint8_t> plain) const {
  uint8_t digest[SHA512_DIGEST_LENGTH];
  unsigned int digest_len = 0;
  if (!HMAC(EVP_sha512(), this->_key.data(), this->_key.size(), plain.data(),
            plain.size(), digest, &digest_len)) {
    spdlog::error("sign hmac");
    return {};
  }
  std::vector<uint8_t> it(std::begin(digest), std::end(digest));
  return it;
}

std::string palm::ssha512::sign(const std::vector<uint8_t> plain,
                                const std::vector<uint8_t> salt) {
  std::vector<uint8_t> buf;
  buf.insert(buf.end(), plain.begin(), plain.end());
  buf.insert(buf.end(), salt.begin(), salt.end());

  auto digest = palm::sha512::sign(buf);
  digest.insert(digest.end(), salt.begin(), salt.end());
  return HEADER + cppcodec::base64_rfc4648::encode(digest);
}

bool palm::ssha512::verify(const std::string &code,
                           const std::vector<uint8_t> plain) {
  if (!code.starts_with(HEADER)) {
    return false;
  }
  std::vector<uint8_t> buf =
      cppcodec::base64_rfc4648::decode(code.substr(HEADER.size()));
  if (buf.size() <= SHA512_DIGEST_LENGTH) {
    return false;
  }

  const std::vector<uint8_t> salt = {buf.begin() + SHA512_DIGEST_LENGTH,
                                     buf.end()};
  return code == sign(plain, salt);
}

// https://wiki.openssl.org/index.php/EVP_Message_Digests
std::vector<uint8_t> palm::sha256::sign(const std::vector<uint8_t> plain) {
  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  if (ctx == nullptr) {
    spdlog::error("new openssl evp context");
    return {};
  }

  auto md = EVP_sha256();
  if (md == nullptr) {
    spdlog::error("init openssl sha256 evp");
    return {};
  }

  if (1 != EVP_DigestInit_ex(ctx, md, nullptr)) {
    spdlog::error("init openssl digest");
    return {};
  };

  if (1 != EVP_DigestUpdate(ctx, plain.data(), plain.size())) {
    spdlog::error("update openssl digest");
    return {};
  };

  uint8_t digest[SHA256_DIGEST_LENGTH];
  unsigned int digest_len = 0;
  if (1 != EVP_DigestFinal_ex(ctx, digest, &digest_len)) {
    spdlog::error("final openssl digest");
    return {};
  };

  EVP_MD_CTX_free(ctx);

  std::vector<uint8_t> it(std::begin(digest), std::end(digest));

  return it;
}

std::vector<uint8_t> palm::sha512::sign(const std::vector<uint8_t> plain) {
  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  if (ctx == nullptr) {
    spdlog::error("new openssl evp context");
    return {};
  }

  auto md = EVP_sha512();
  if (md == nullptr) {
    spdlog::error("init openssl sha512 evp");
    return {};
  }

  if (1 != EVP_DigestInit_ex(ctx, md, nullptr)) {
    spdlog::error("init openssl digest");
    return {};
  };

  if (1 != EVP_DigestUpdate(ctx, plain.data(), plain.size())) {
    spdlog::error("update openssl digest");
    return {};
  };

  uint8_t digest[SHA512_DIGEST_LENGTH];
  unsigned int digest_len = 0;
  if (1 != EVP_DigestFinal_ex(ctx, digest, &digest_len)) {
    spdlog::error("final openssl digest");
    return {};
  };

  EVP_MD_CTX_free(ctx);

  std::vector<uint8_t> it(std::begin(digest), std::end(digest));
  return it;
}

std::optional<std::string> palm::md5(const std::filesystem::path &filename) {
  std::ifstream file(filename, std::ios::binary);
  if (!file.is_open()) {
    return std::nullopt;
  }

  EVP_MD_CTX *md_ctx = EVP_MD_CTX_new();
  if (!md_ctx) {
    spdlog::error("init openssl evp");
    return std::nullopt;
  }
  if (1 != EVP_DigestInit_ex(md_ctx, EVP_md5(), NULL)) {
    spdlog::error("init openssl digest");
    return std::nullopt;
  }

  std::vector<char> buffer(4096);
  while (file.read(buffer.data(), buffer.size())) {
    if (1 != EVP_DigestUpdate(md_ctx, buffer.data(), file.gcount())) {
      spdlog::error("update evp digest");
      return std::nullopt;
    }
  }

  if (1 != EVP_DigestUpdate(md_ctx, buffer.data(), file.gcount())) {
    spdlog::error("update evp digest(tail)");
    return std::nullopt;
  }
  unsigned char md_value[EVP_MAX_MD_SIZE];
  unsigned int md_len;
  if (1 != EVP_DigestFinal_ex(md_ctx, md_value, &md_len)) {
    spdlog::error("final evp digest");
    return std::nullopt;
  }
  EVP_MD_CTX_free(md_ctx);

  std::stringstream ss;
  for (unsigned int i = 0; i < md_len; ++i) {
    ss << std::hex << std::setw(2) << std::setfill('0')
       << static_cast<int>(md_value[i]);
  }
  return ss.str();
}
