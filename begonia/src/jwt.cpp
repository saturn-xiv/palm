#include "palm/jwt.hpp"

#include <algorithm>

#include <jwt-cpp/jwt.h>
#include <spdlog/spdlog.h>
#include <nlohmann/json.hpp>

std::string palm::Jwt::sign(
    const std::optional<std::string> jwt_id,
    const std::optional<std::string> key_id, const std::string& issuer,
    const std::string& subject, const std::set<std::string> audiences,
    const std::chrono::time_point<std::chrono::system_clock>& issued_at,
    const std::chrono::time_point<std::chrono::system_clock>& not_before,
    const std::chrono::time_point<std::chrono::system_clock>& expired_at,
    const std::optional<std::string> payload) const {
  spdlog::debug("generate token for ({}, {})", issuer, subject);

  auto builder = jwt::create()
                     .set_type("JWS")
                     .set_issuer(issuer)
                     .set_subject(subject)
                     //   .set_audience(audiences)
                     .set_issued_at(issued_at)
                     .set_not_before(not_before)
                     .set_expires_at(expired_at);
  if (jwt_id) {
    builder = builder.set_id(jwt_id.value());
  }
  if (key_id) {
    builder = builder.set_key_id(key_id.value());
  }
  if (payload) {
    builder = builder.set_payload_claim(PAYLOAD_CLAIM_KEY,
                                        jwt::claim(payload.value()));
  }
  {
    nlohmann::json js = audiences;
    builder = builder.set_audience(js.dump());
  }

  return builder.sign(jwt::algorithm::hs512{this->_key});
}

static bool has_audience(const std::set<std::string> audiences,
                         const std::string& audience) {
  for (const auto& it : audiences) {
    const auto js = nlohmann::json::parse(it);
    const auto tmp = js.get<std::vector<std::string>>();
    if (std::find(tmp.begin(), tmp.end(), audience) != tmp.end()) {
      return true;
    }
  }
  return false;
}

std::tuple<std::optional<std::string>, std::optional<std::string>, std::string,
           std::optional<std::string>>
palm::Jwt::verify(const std::string& token, const std::string& issuer,
                  const std::string& audience) const {
  auto decoded = jwt::decode(token);
  auto verifier = jwt::verify()
                      .with_issuer(issuer)
                      //   .with_audience(audience)
                      .allow_algorithm(jwt::algorithm::hs512{this->_key});

  verifier.verify(decoded);
  if (!has_audience(decoded.get_audience(), audience)) {
    throw jwt::error::signature_verification_exception();
  }
  //   {
  //     const std::set<std::string> it = decoded.get_audience();
  //     if (it.size() != 1) {
  //       throw jwt::error::signature_verification_exception();
  //     }
  //     const std::string v = it.at;

  //     // for (const auto it : decoded.get_audience()) {
  //     //   spdlog::debug( "------- {}", it);
  //     // }
  //     // if (!.contains(audience)) {
  //     //   // throw jwt::error::signature_verification_exception();
  //     // }
  //   }

  std::optional<std::string> jwt_id =
      decoded.has_id() ? std::optional<std::string>{decoded.get_id()}
                       : std::nullopt;
  std::optional<std::string> key_id =
      decoded.has_key_id() ? std::optional<std::string>{decoded.get_key_id()}
                           : std::nullopt;

  std::string subject = decoded.get_subject();
  std::optional<std::string> payload =
      decoded.has_payload_claim(PAYLOAD_CLAIM_KEY)
          ? std::optional<std::string>{decoded
                                           .get_payload_claim(PAYLOAD_CLAIM_KEY)
                                           .as_string()}
          : std::nullopt;
  return {jwt_id, key_id, subject, payload};
}
