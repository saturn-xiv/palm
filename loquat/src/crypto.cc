#include "env.hpp"

#include "loquat/crypto.hpp"
#include "loquat/version.hpp"

#include <tink/config/tink_config.h>
#include <tink/jwt/jwt_mac_config.h>
#include <tink/version.h>

static std::shared_ptr<loquat::Aes> gl_aes;
static std::shared_ptr<loquat::HMac> gl_hmac;
static std::shared_ptr<loquat::Jwt> gl_jwt;

std::string LOQUAT_LIB_API loquat::aes::encrypt(const std::string& plain) {
  return gl_aes->encrypt(plain);
}

std::string LOQUAT_LIB_API loquat::aes::decrypt(const std::string& code) {
  return gl_aes->decrypt(code);
}

std::string LOQUAT_LIB_API loquat::hmac::sign(const std::string& plain) {
  return gl_hmac->sign(plain);
}
void LOQUAT_LIB_API loquat::hmac::verify(const std::string& code,
                                         const std::string& plain) {
  gl_hmac->verify(code, plain);
}

std::tuple<std::optional<std::string>, std::optional<std::string>, std::string,
           std::optional<std::string>>
    LOQUAT_LIB_API loquat::jwt::verify(const std::string& token,
                                       const std::string& issuer,
                                       const std::string& audience) {
  return gl_jwt->verify(token, issuer, audience);
}

std::string LOQUAT_LIB_API
loquat::jwt::sign(const std::string& issuer, const std::string& subject,
                  const std::set<std::string> audiences, uint32_t ttl,
                  const std::optional<std::string> payload) {
  const auto now = absl::Now();
  const auto nbf = now - absl::Seconds(1);
  const auto exp = now + absl::Minutes(static_cast<int>(ttl));

  return gl_jwt->sign(std::nullopt, std::nullopt, issuer, subject, audiences,
                      now, nbf, exp, payload);
}

int LOQUAT_LIB_API loquat::init(const std::string& namespace_, bool debug) {
  {
    spdlog::set_level(debug ? spdlog::level::debug : spdlog::level::info);
    spdlog::debug("init on debug mode {}", loquat::GIT_VERSION);
    spdlog::debug("Tink v{}", crypto::tink::Version::kTinkVersion);
  }
  {
    const auto status = crypto::tink::TinkConfig::Register();
    if (!status.ok()) {
      spdlog::error("failed to register tink");
      return EXIT_FAILURE;
    }
  }
  {
    const auto status = crypto::tink::JwtMacRegister();
    if (!status.ok()) {
      spdlog::error("failed to register tink-jwt");
      return EXIT_FAILURE;
    }
  }

  gl_aes = std::make_shared<loquat::Aes>(std::format("{}.aes", namespace_));
  gl_hmac = std::make_shared<loquat::HMac>(std::format("{}.hmac", namespace_));
  gl_jwt = std::make_shared<loquat::Jwt>(std::format("{}.jwt", namespace_));

  return EXIT_SUCCESS;
}
