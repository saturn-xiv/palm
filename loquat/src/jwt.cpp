#include "loquat/env.hpp"

#include <tink/jwt/internal/jwt_mac_impl.h>
#include <tink/jwt/internal/jwt_mac_internal.h>
#include <tink/jwt/jwt_key_templates.h>
#include <tink/jwt/jwt_signature_config.h>
#include <tink/jwt/jwt_validator.h>
#include <tink/jwt/raw_jwt.h>

std::optional<std::string> loquat::Jwt::sign(
    const std::optional<std::string> jwt_id,
    const std::optional<std::string> key_id, const std::string& issuer,
    const std::string& subject, const std::set<std::string> audiences,
    const absl::Time& issued_at, const absl::Time& not_before,
    const absl::Time& expired_at, const std::optional<std::string> payload) {
  spdlog::debug(
      "sign token for jwt-id({}) key-id({}) issuer({}) subject({}) "
      "audiences({})",
      jwt_id.value_or(""), key_id.value_or(""), issuer, subject,
      absl::StrJoin(audiences, ","));
  // https://github.com/tink-crypto/tink-cc/blob/main/tink/jwt/raw_jwt.h#L101
  auto raw_rb = crypto::tink::RawJwtBuilder()
                    .SetIssuer(issuer)
                    .SetSubject(subject)
                    .SetNotBefore(not_before)
                    .SetIssuedAt(issued_at)
                    .SetExpiration(expired_at);
  if (jwt_id) {
    raw_rb = raw_rb.SetJwtId(jwt_id.value());
  }
  for (const auto& it : audiences) {
    raw_rb = raw_rb.AddAudience(it);
  }
  if (payload) {
    raw_rb =
        raw_rb.AddJsonObjectClaim(loquat::Jwt::PAYLOAD_KEY, payload.value());
  }

  auto raw_r = raw_rb.Build();
  {
    const auto status = raw_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto raw = std::move(raw_r.value());
  auto jwt = this->load();
  auto token_r = jwt->ComputeMacAndEncode(raw);
  {
    const auto status = token_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto token = std::move(token_r.value());
  return token;
}

std::optional<std::pair<std::string, std::optional<std::string>>>
loquat::Jwt::verify(const std::string& token, const std::string& issuer,
                    const std::string& audience) {
  spdlog::debug("verify issuer({}) audience({}) token({})", issuer, audience,
                token);
  auto validator_b = crypto::tink::JwtValidatorBuilder()
                         .IgnoreTypeHeader()
                         .ExpectIssuer(issuer)
                         .ExpectAudience(audience)
                         .ExpectIssuedInThePast();

  auto validator_r = validator_b.Build();
  {
    const auto status = validator_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto validator = std::move(validator_r.value());

  auto jwt = this->load();
  auto payload_r = jwt->VerifyMacAndDecode(token, validator);
  {
    const auto status = payload_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto payload = std::move(payload_r.value());

  // https://github.com/tink-crypto/tink-cc/blob/main/tink/jwt/verified_jwt.h#L53
  auto subject_r = payload.GetSubject();
  {
    const auto status = subject_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return std::nullopt;
    }
  }
  auto subject = std::move(subject_r.value());

  std::optional<std::string> jwt_id = std::nullopt;
  if (payload.HasJwtId()) {
    auto ir = payload.GetJwtId();
    {
      const auto status = ir.status();
      if (!status.ok()) {
        spdlog::error("{}", status.message());
        return std::nullopt;
      }
    }
    auto iv = std::move(ir.value());
    jwt_id = std::optional<std::string>{iv};
  }

  std::optional<std::string> payload_ = std::nullopt;
  if (payload.HasJsonObjectClaim(loquat::Jwt::PAYLOAD_KEY)) {
    auto ir = payload.GetJsonObjectClaim(loquat::Jwt::PAYLOAD_KEY);
    {
      const auto status = ir.status();
      if (!status.ok()) {
        spdlog::error("{}", status.message());
        return std::nullopt;
      }
    }
    auto iv = std::move(ir.value());
    payload_ = std::optional<std::string>{iv};
  }

  spdlog::debug("get jwt-id({}) subject({})", jwt_id.value_or(""), subject);
  return std::make_pair(subject, payload_);
}

std::unique_ptr<crypto::tink::JwtMac> loquat::Jwt::load() {
  auto keyset = this->Keyset::load(crypto::tink::JwtHs512Template());
  auto jwt_r = keyset->GetPrimitive<crypto::tink::JwtMac>(
      crypto::tink::ConfigGlobalRegistry());
  {
    const auto status = jwt_r.status();
    if (!status.ok()) {
      spdlog::error("{}", status.message());
      return nullptr;
    }
  }
  auto jwt = std::move(jwt_r.value());
  return jwt;
}
