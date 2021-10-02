#pragma once

#include "palm/env.hpp"

#include "Poco/JWT/Signer.h"

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

class Session {
 public:
  Session(grpc::ServerContext* ctx);
  friend std::ostream& operator<<(std::ostream& out, const Session& self) {
    out << "peer => " << self.peer;
    if (self.token) {
      out << "\ntoken => " << self.token.value();
    }
    return out;
  }

  inline const static std::string UID = "uid";
  inline const static std::string AUTHORIZAATION = "authorization";
  inline const static std::string BEARER = "Bearer ";
  inline static void authorization(grpc::ClientContext* ctx,
                                   const std::string& token) {
    ctx->AddMetadata(AUTHORIZAATION, BEARER + token);
  }

  // https://datatracker.ietf.org/doc/html/rfc7519#section-4.1
  inline std::optional<std::string> current_user(
      std::shared_ptr<Poco::JWT::Signer> signer) const {
    if (this->token) {
      const auto token = signer->verify(this->token.value());
      Poco::Timestamp now;
      if (now > token.getNotBefore() && now < token.getExpiration()) {
        return token.getSubject();
      }
    }

    return std::nullopt;
  }

 private:
  std::optional<std::string> token;
  std::string peer;
};

}  // namespace palm
