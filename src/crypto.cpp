#include "palm/crypto.hpp"

#include <boost/algorithm/string.hpp>
#include <boost/beast/core/detail/base64.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#include <jwt-cpp/jwt.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/hmac.h>
#include <openssl/opensslv.h>
#include <openssl/pem.h>
#include <openssl/sha.h>
#include <sodium.h>
#include <cppcodec/base64_rfc4648.hpp>
#include <cppcodec/base64_url.hpp>

std::string palm::url::encode(const std::vector<uint8_t>& buf) {
  return cppcodec::base64_url::encode(buf);
}
std::string palm::url::encode(const std::string& buf) {
  return cppcodec::base64_url::encode(buf);
}

std::string palm::base64::encode(const std::vector<uint8_t>& buf) {
  return cppcodec::base64_rfc4648::encode(buf);
}

std::vector<uint8_t> palm::base64::decode(const std::string& buf) {
  return cppcodec::base64_rfc4648::decode(buf);
}

static void check_openssl_err(const std::string& act) {
  ERR_print_errors_fp(stderr);
  throw std::invalid_argument("openssl " + act);
}

int palm::Aes::encrypt(const uint8_t* plain, const int plain_len,
                       const uint8_t* key, const uint8_t* iv,
                       uint8_t* cipher) const {
  EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
  if (ctx == NULL) {
    check_openssl_err("init cipher");
  }

  if (1 != EVP_EncryptInit_ex(ctx, EVP_aes_256_cbc(), NULL, key, iv)) {
    check_openssl_err("init encrypt ex");
  }

  int cipher_len = 0;
  int len = 0;
  if (1 != EVP_EncryptUpdate(ctx, cipher, &len, plain, plain_len)) {
    check_openssl_err("encrypt update");
  }
  cipher_len += len;

  if (1 != EVP_EncryptFinal_ex(ctx, cipher + len, &len)) {
    check_openssl_err("encrypt update");
  }
  cipher_len += len;

  EVP_CIPHER_CTX_free(ctx);

  return cipher_len;
}

// https://wiki.openssl.org/index.php/EVP_Symmetric_Encryption_and_Decryption
int palm::Aes::decrypt(const uint8_t* cipher, const size_t cipher_len,
                       const uint8_t* key, const uint8_t* iv,
                       uint8_t* plain) const {
  EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
  if (ctx == NULL) {
    check_openssl_err("init cipher ctx");
  }

  if (1 != EVP_DecryptInit_ex(ctx, EVP_aes_256_cbc(), NULL, key, iv)) {
    check_openssl_err("init decrypt ex");
  }

  int len = 0;
  int plain_len = 0;
  if (1 != EVP_DecryptUpdate(ctx, plain, &len, cipher, cipher_len)) {
    check_openssl_err("decrypt update");
  }
  plain_len += len;

  if (1 != EVP_DecryptFinal_ex(ctx, plain + len, &len)) {
    check_openssl_err("decrypt final ex");
  }
  plain_len += len;
  // plain[plain_len] = '\0';

  EVP_CIPHER_CTX_free(ctx);

  return plain_len;
}
std::pair<std::vector<uint8_t>, std::vector<uint8_t>> palm::Aes::encrypt(
    const std::string& input) const {
  const auto key = palm::base64::decode(this->key);
  const auto iv = palm::random::bytes(16);
  std::vector<uint8_t> code(1 << 10);
  const auto len = this->encrypt((uint8_t*)input.c_str(), input.size(),
                                 &key.front(), &iv.front(), &code.front());
  code.resize(len);
  return std::make_pair(code, iv);
}
std::string palm::Aes::decrypt(const std::vector<uint8_t>& input,
                               const std::vector<uint8_t>& iv) const {
  const auto key = palm::base64::decode(this->key);
  std::vector<uint8_t> plain(1 << 10);
  const auto len = this->decrypt(&input.front(), input.size(), &key.front(),
                                 &iv.front(), &plain.front());
  plain.resize(len);
  std::string buf(plain.begin(), plain.end());
  return buf;
}

std::string palm::Hmac::sum(const std::string& plain,
                            const std::vector<uint8_t>& salt) const {
  const auto key = palm::base64::decode(this->key);
  const EVP_MD* engine = EVP_sha512();

  auto ctx = HMAC_CTX_new();
  if (!HMAC_Init_ex(ctx, &key.front(), key.size(), engine, NULL)) {
    std::stringstream ss;
    ss << "sha512-hmac init (" << key.size() << ") " << this->key;
    throw std::invalid_argument(ss.str());
  }
  if (!HMAC_Update(ctx, (uint8_t*)plain.c_str(), plain.size())) {
    throw std::invalid_argument("sha512-hmac update password");
  }
  if (!HMAC_Update(ctx, &salt.front(), salt.size())) {
    throw std::invalid_argument("sha512-hmac update salt");
  }

  std::vector<uint8_t> sha(EVP_MAX_MD_SIZE);
  unsigned int len = 0;
  if (!HMAC_Final(ctx, &sha.front(), &len)) {
    throw std::invalid_argument("sha512-hmac final");
  }

  std::vector<uint8_t> buf;
  buf.insert(buf.end(), sha.begin(), sha.end());
  buf.insert(buf.end(), salt.begin(), salt.end());
  HMAC_CTX_free(ctx);

  return palm::base64::encode(buf);
}
std::string palm::Hmac::sum(const std::string& plain,
                            const size_t salt_len) const {
  const auto salt = palm::random::bytes(salt_len);
  return this->sum(plain, salt);
}

bool palm::Hmac::verify(const std::string& password,
                        const std::string& plain) const {
  const auto buf = palm::base64::decode(password);
  const std::vector<uint8_t> salt = {buf.begin() + EVP_MAX_MD_SIZE, buf.end()};
  return this->sum(plain, salt) == password;
}

std::unordered_map<std::string, std::string> palm::Jwt::parse(
    const std::string& token) const {
  auto decoded = jwt::decode(token);
  auto verifier = jwt::verify()
                      .allow_algorithm(jwt::algorithm::hs512{this->key})
                      .with_issuer(this->issuer);

  verifier.verify(decoded);

  std::unordered_map<std::string, std::string> payload;
  for (const auto& it : decoded.get_payload_claims()) {
    if (it.second.get_type() == jwt::json::type::string) {
      payload[it.first] = it.second.as_string();
    }
  }
  return payload;
}

std::string palm::Jwt::sum(
    const std::unordered_map<std::string, std::string>& payload,
    const std::chrono::seconds& ttl) const {
  const auto now = std::chrono::system_clock::now();

  auto jwt = jwt::create()
                 .set_issuer(this->issuer)
                 .set_issued_at(now)
                 .set_not_before(now)
                 .set_expires_at(now + ttl);
  for (const auto& it : payload) {
    jwt.set_payload_claim(it.first, jwt::claim(it.second));
  }
  return jwt.sign(jwt::algorithm::hs512{this->key});
}

// https://github.com/pokowaka/jwt-cpp/blob/b1db67e54f01f72c914af82aaea9a8d49d6dc6b9/src/validators/hmacvalidator.cpp
// https://github.com/pokowaka/jwt-cpp/blob/master/src/include/private/ssl_compat.h
// https://github.com/pokowaka/jwt-cpp/blob/b1db67e54f01f72c914af82aaea9a8d49d6dc6b9/src/jwt/jwt.cpp

std::string palm::uuid::v4() {
  // std::ifstream it("/proc/sys/kernel/random/uuid");
  // trim
  // return std::string((std::istreambuf_iterator<char>(it)),
  //                    std::istreambuf_iterator<char>());
  boost::uuids::uuid it = boost::uuids::random_generator()();
  return boost::uuids::to_string(it);
}

std::string palm::random::string(const size_t len) {
  std::stringstream ss;
  std::uniform_int_distribution<int> ud(32, 126);
  std::random_device rd;
  for (auto i = 0; i < len; i++) {
    ss << (char)ud(rd);
  }
  return ss.str();
}

std::string palm::random::alphanumeric(const size_t len) {
  static auto& line =
      R"(0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ)";
  thread_local static std::mt19937 rg{std::random_device{}()};
  thread_local static std::uniform_int_distribution<std::string::size_type>
      pick(0, sizeof(line) - 2);

  std::stringstream ss;
  for (auto i = 0; i < len; i++) {
    ss << line[pick(rg)];
  }
  return ss.str();
}

std::vector<uint8_t> palm::random::bytes(const size_t len) {
  std::random_device rd;
  static std::independent_bits_engine<std::default_random_engine, CHAR_BIT,
                                      uint8_t>
      rbe(rd());
  std::vector<uint8_t> buf(len);
  std::generate(begin(buf), end(buf), std::ref(rbe));
  return buf;
}

palm::Session::Session(grpc::ServerContext* ctx) {
  this->peer = ctx->peer();

  const auto mt = ctx->client_metadata();
  {
    const auto it = mt.find(AUTHORIZAATION);
    if (it != mt.end()) {
      if (it->second.starts_with(BEARER)) {
        const auto auth = it->second.substr(BEARER.length());
        this->token = std::string(auth.data());
      }
    }
  }
}

std::optional<std::string> palm::Session::current_user(
    std::shared_ptr<palm::Jwt> jwt) const {
  if (this->token) {
    auto payload = jwt->parse(this->token.value());
    return payload[UID];
  }

  return std::nullopt;
}
