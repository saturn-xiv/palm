#include "palm/crypto.hpp"

#include <boost/log/trivial.hpp>

#include <openssl/evp.h>
#include <cppcodec/base64_rfc4648.hpp>

// https://developers.google.com/tink/key-concepts
// https://wiki.openssl.org/index.php/EVP_Symmetric_Encryption_and_Decryption
palm::Aes::Aes(const std::string &key, const std::string &iv)
    : _key(cppcodec::base64_rfc4648::decode(key)),
      _iv(cppcodec::base64_rfc4648::decode(iv)) {
  if (this->_key.size() != 256 / 8) {
    throw std::invalid_argument("aes key");
  }
  if (this->_iv.size() != 128 / 8) {
    throw std::invalid_argument("aes iv");
  }
  EVP_add_cipher(EVP_aes_256_cbc());
}

std::vector<uint8_t> palm::Aes::encrypt(
    const std::vector<uint8_t> plain) const {
  EVP_CIPHER_CTX *ctx = EVP_CIPHER_CTX_new();
  if (ctx == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "init openssl cipher ctx";
    return {};
  }

  if (1 != EVP_EncryptInit_ex(ctx, EVP_aes_256_cbc(), NULL, this->_key.data(),
                              this->_iv.data())) {
    BOOST_LOG_TRIVIAL(error) << "init openssl aes-268-cbc";
    return {};
  }

  std::vector<uint8_t> buf;
  buf.reserve(BUFFER_MAX_SIZE);

  int cipher_len = 0;

  {
    int len = 0;
    if (1 !=
        EVP_EncryptUpdate(ctx, buf.data(), &len, plain.data(), plain.size())) {
      BOOST_LOG_TRIVIAL(error) << "update encrypt";
      return {};
    }
    cipher_len += len;
  }

  {
    int len = 0;
    if (1 != EVP_EncryptFinal_ex(ctx, buf.data() + cipher_len, &len)) {
      BOOST_LOG_TRIVIAL(error) << "finial encrypt";
      return {};
    }
    cipher_len += len;
  }

  EVP_CIPHER_CTX_free(ctx);

  const std::vector<uint8_t> cipher = {buf.begin(), buf.begin() + cipher_len};
  return cipher;
}
std::vector<uint8_t> palm::Aes::decrypt(
    const std::vector<uint8_t> code) const {
  EVP_CIPHER_CTX *ctx = EVP_CIPHER_CTX_new();

  if (ctx == nullptr) {
    BOOST_LOG_TRIVIAL(error) << "init openssl cipher ctx";
    return {};
  }

  if (1 != EVP_DecryptInit_ex(ctx, EVP_aes_256_cbc(), NULL, this->_key.data(),
                              this->_iv.data())) {
    BOOST_LOG_TRIVIAL(error) << "init decrypt";
    return {};
  }

  std::vector<uint8_t> buf;
  buf.reserve(BUFFER_MAX_SIZE);

  int plain_len = 0;

  {
    int len = 0;
    if (1 !=
        EVP_DecryptUpdate(ctx, buf.data(), &len, code.data(), code.size())) {
      BOOST_LOG_TRIVIAL(error) << "update decrypt";
      return {};
    }
    plain_len += len;
  }

  {
    int len = 0;
    if (1 != EVP_DecryptFinal_ex(ctx, buf.data() + plain_len, &len)) {
      BOOST_LOG_TRIVIAL(error) << "finial decrypt";
      return {};
    }
    plain_len += len;
  }

  EVP_CIPHER_CTX_free(ctx);

  const std::vector<uint8_t> plain = {buf.begin(), buf.begin() + plain_len};
  return plain;
}
