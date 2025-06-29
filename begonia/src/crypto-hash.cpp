#include "basil/crypto.hpp"

#include <boost/algorithm/hex.hpp>
#include <boost/log/trivial.hpp>

#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/sha.h>
#include <cppcodec/base64_rfc4648.hpp>

basil::HMac::HMac(const std::string &key)
    : _key(cppcodec::base64_rfc4648::decode(key)) {}
std::vector<uint8_t> basil::HMac::sign(const std::vector<uint8_t> plain) const {
  uint8_t digest[SHA512_DIGEST_LENGTH];
  unsigned int digest_len = 0;
  if (!HMAC(EVP_sha512(), this->_key.data(), this->_key.size(), plain.data(),
            plain.size(), digest, &digest_len)) {
    BOOST_LOG_TRIVIAL(error) << "sign hmac";
    return {};
  }
  std::vector<uint8_t> it(std::begin(digest), std::end(digest));
  return it;
}

std::string basil::ssha512::sign(const std::vector<uint8_t> plain,
                                 const std::vector<uint8_t> salt) {
  std::vector<uint8_t> buf;
  buf.insert(buf.end(), plain.begin(), plain.end());
  buf.insert(buf.end(), salt.begin(), salt.end());

  auto digest = basil::sha512::sign(buf);
  digest.insert(digest.end(), salt.begin(), salt.end());
  return HEADER + cppcodec::base64_rfc4648::encode(digest);
}

bool basil::ssha512::verify(const std::string &code,
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
std::vector<uint8_t> basil::sha256::sign(const std::vector<uint8_t> plain) {
  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  if (ctx == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "new openssl evp context";
    return {};
  }

  auto md = EVP_sha256();
  if (md == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "init openssl sha256 evp";
    return {};
  }

  if (1 != EVP_DigestInit_ex(ctx, md, nullptr)) {
    BOOST_LOG_TRIVIAL(error) << "init openssl digest";
    return {};
  };

  if (1 != EVP_DigestUpdate(ctx, plain.data(), plain.size())) {
    BOOST_LOG_TRIVIAL(error) << "update openssl digest";
    return {};
  };

  uint8_t digest[SHA256_DIGEST_LENGTH];
  unsigned int digest_len = 0;
  if (1 != EVP_DigestFinal_ex(ctx, digest, &digest_len)) {
    BOOST_LOG_TRIVIAL(error) << "final openssl digest";
    return {};
  };

  EVP_MD_CTX_free(ctx);

  std::vector<uint8_t> it(std::begin(digest), std::end(digest));

  return it;
}

std::vector<uint8_t> basil::sha512::sign(const std::vector<uint8_t> plain) {
  EVP_MD_CTX *ctx = EVP_MD_CTX_new();
  if (ctx == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "new openssl evp context";
    return {};
  }

  auto md = EVP_sha512();
  if (md == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "init openssl sha512 evp";
    return {};
  }

  if (1 != EVP_DigestInit_ex(ctx, md, nullptr)) {
    BOOST_LOG_TRIVIAL(error) << "init openssl digest";
    return {};
  };

  if (1 != EVP_DigestUpdate(ctx, plain.data(), plain.size())) {
    BOOST_LOG_TRIVIAL(error) << "update openssl digest";
    return {};
  };

  uint8_t digest[SHA512_DIGEST_LENGTH];
  unsigned int digest_len = 0;
  if (1 != EVP_DigestFinal_ex(ctx, digest, &digest_len)) {
    BOOST_LOG_TRIVIAL(error) << "final openssl digest";
    return {};
  };

  EVP_MD_CTX_free(ctx);

  std::vector<uint8_t> it(std::begin(digest), std::end(digest));
  return it;
}
