#pragma once

#include "palm/env.hpp"

#include <openssl/evp.h>

namespace palm {

std::string timestamp();
namespace uuid {
std::string v4();
}

namespace random {
std::string string(std::string::size_type length);
std::vector<uint8_t> bytes(const size_t length);
}  // namespace random

namespace hex {
std::string to(const std::vector<uint8_t>& buffer);
std::vector<uint8_t> from(const std::string& buffer);
}  // namespace hex

namespace base64 {
std::string to(const std::vector<uint8_t>& buffer);
std::vector<uint8_t> from(const std::string& buffer);
}  // namespace base64

// class Aes {
//  public:
//   Aes(const std::string& key) : key(key) {}
//   ~Aes() {}
//   std::pair<std::vector<uint8_t>, std::vector<uint8_t>> encrypt(
//       const std::string& input) const;
//   std::string decrypt(const std::vector<uint8_t>& input,
//                       const std::vector<uint8_t>& iv) const;

//  private:
//   int encrypt(const uint8_t* plain, const int plain_len, const uint8_t* key,
//               const uint8_t* iv, uint8_t* cipher) const;
//   int decrypt(const uint8_t* cipher, const size_t cipher_len,
//               const uint8_t* key, const uint8_t* iv, uint8_t* plain) const;

//   const std::string key;
// };

// class Jwt {
//  public:
//   Jwt(const std::string& key, const std::string& issuer)
//       : key(key), issuer(issuer) {}

//   std::unordered_map<std::string, std::string> parse(
//       const std::string& token) const;

//   std::string sum(const std::unordered_map<std::string, std::string>&
//   payload,
//                   const std::chrono::seconds& ttl) const;

//  private:
//   const std::string key;
//   const std::string issuer;
// };

// class Hmac {
//  public:
//   Hmac(const std::string& key) : key(key) {}
//   ~Hmac() {}
//   std::string sum(const std::string& plain, const size_t salt_len) const;
//   bool verify(const std::string& password, const std::string& plain) const;

//  private:
//   std::string sum(const std::string& plain,
//                   const std::vector<uint8_t>& salt) const;

//   const std::string key;
// };

// inline const static std::string AUTHORIZAATION = "authorization";
// inline const static std::string BEARER = "Bearer ";
// inline void authorization(grpc::ClientContext* ctx, const std::string& token)
// {
//   ctx->AddMetadata(AUTHORIZAATION, BEARER + token);
// }

// class Session {
//  public:
//   Session(grpc::ServerContext* ctx);
//   friend std::ostream& operator<<(std::ostream& out, const Session& self) {
//     out << "peer => " << self.peer;
//     if (self.token) {
//       out << "\ntoken => " << self.token.value();
//     }
//     return out;
//   }

//   std::optional<std::string> current_user(std::shared_ptr<palm::Jwt> jwt)
//   const; inline const static std::string UID = "uid";

//  private:
//   std::optional<std::string> token;
//   std::string peer;
// };

}  // namespace palm
